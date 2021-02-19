mod atom;
mod command;
mod decompress;
mod download;
mod echo;
mod gist;
mod http;
mod parser;
mod util;
mod wechat;

use crate::atom::Atom;
use crate::command::Command;
use crate::decompress::Decompress;
use crate::download::Download;
use crate::echo::Echo;
use crate::gist::Gist;
use crate::http::Http;
use crate::wechat::WeChat;
use anyhow::{anyhow, Context as _, Result};
use enum_dispatch::enum_dispatch;
use lazy_static::lazy_static;
use parser::fulfill;
use serde::Deserialize;
use std::{collections::HashMap, env, fs};

const USER_AGENT: &'static str = "workflows/1.0";

#[enum_dispatch(SupportedWorkflows)]
trait Workflow {
    fn execute<T>(&self, context: Context, input: Inputs, next: T) -> Result<()>
    where
        T: FnOnce(Context, Outputs) -> Result<()>;
    fn parameters(&self) -> &'static [&'static str];
    fn outputs(&self) -> &'static [&'static str];
}

#[derive(Debug)]
pub struct Context {
    config: Config,
    env: HashMap<String, String>,
}

impl Context {
    fn new(config: Config) -> Self {
        let env: HashMap<String, String> = env::vars().collect();

        Self { config, env }
    }

    fn next(&mut self) -> Option<WorkflowConfig> {
        if self.config.workflows.is_empty() {
            None
        } else {
            Some(self.config.workflows.remove(0))
        }
    }
}

type Outputs = HashMap<&'static str, String>;
type Inputs = HashMap<&'static str, String>;

trait Input {
    fn parameter(&self, key: &'static str) -> &str;
}

impl Input for Inputs {
    fn parameter(&self, key: &'static str) -> &str {
        self.get(key).map(|s| &s[..]).unwrap_or_else(|| "")
    }
}

#[enum_dispatch]
enum SupportedWorkflows {
    Http,
    Gist,
    Echo,
    WeChat,
    Command,
    Download,
    Decompress,
    Atom,
}

lazy_static! {
    static ref WORKFLOWS: HashMap<&'static str, SupportedWorkflows> = {
        let mut m = HashMap::new();
        m.insert("http", Http {}.into());
        m.insert("echo", Echo {}.into());
        m.insert("wechat", WeChat {}.into());
        m.insert("gist", Gist {}.into());
        m.insert("command", Command {}.into());
        m.insert("download", Download {}.into());
        m.insert("decompress", Decompress {}.into());
        m.insert("atom", Atom {}.into());
        m
    };
}

#[derive(Debug, Deserialize)]
struct Config {
    workflows: Vec<WorkflowConfig>,
}
#[derive(Debug, Deserialize)]
struct WorkflowConfig {
    #[serde(rename = "type")]
    workflow_type: String,
    parameters: HashMap<String, String>,
}

fn make_workflow(
    config: &WorkflowConfig,
    input: &HashMap<String, String>,
    context: &Context,
) -> Result<(&'static SupportedWorkflows, Inputs)> {
    let workflow = WORKFLOWS
        .get(&config.workflow_type.to_lowercase()[..])
        .context(anyhow!("Workflow {} is not found.", config.workflow_type))?;
    let mut payload: HashMap<&'static str, String> = HashMap::new();
    for key in workflow.parameters() {
        if let Some(value) = config.parameters.get(*key) {
            payload.insert(key, fulfill(value, input, &context)?);
        }
    }
    Ok((workflow, payload))
}

fn do_next(mut context: Context, output: Outputs) -> Result<()> {
    let input: HashMap<String, String> = output
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect();
    let workflow_config = context.next();
    if let Some(workflow_config) = workflow_config {
        let (workflow, payload) = make_workflow(&workflow_config, &input, &context)?;
        workflow.execute(context, payload, do_next)?;
    }

    Ok(())
}

fn main() -> Result<()> {
    let config_path = env::args()
        .nth(1)
        .context("No configuration is provided.")?;

    let config = fs::read_to_string(config_path)?;
    let config: Config = serde_yaml::from_str(&config)?;

    let context = Context::new(config);
    do_next(context, HashMap::new())
}
