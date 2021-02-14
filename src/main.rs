mod echo;
mod gist;
mod http;
mod parser;
mod util;
mod wechat;

use crate::echo::Echo;
use crate::gist::Gist;
use crate::http::Http;
use crate::wechat::WeChat;

use anyhow::{anyhow, Context as _, Result};
use enum_dispatch::enum_dispatch;

use parser::fulfill;
use serde::Deserialize;
use std::{collections::HashMap, env, fs};

const USER_AGENT: &'static str = "assemblies/1.0";

#[enum_dispatch(SupportedAssemblies)]
trait Assembly {
    fn assemble(self, input: Payload) -> Result<Payload>;
    fn parameters(&self) -> &'static [&'static str];
    fn outputs(&self) -> &'static [&'static str];
}

#[derive(Debug)]
pub struct Context {
    env: HashMap<String, String>,
    input: HashMap<String, String>,
}

impl Context {
    fn new() -> Self {
        let env: HashMap<String, String> = env::vars().collect::<_>();
        let input: HashMap<String, String> = HashMap::new();

        Self { env, input }
    }
}

#[derive(Debug)]
struct Payload {
    parameters: HashMap<&'static str, String>,
}

impl Payload {
    fn new(parameters: HashMap<&'static str, String>) -> Self {
        Self { parameters }
    }

    fn parameter(&self, key: &'static str) -> &str {
        self.parameters
            .get(key)
            .map(|s| &s[..])
            .unwrap_or_else(|| "")
    }
}

#[enum_dispatch]
enum SupportedAssemblies {
    Http,
    Gist,
    Echo,
    WeChat,
}
#[derive(Debug, Deserialize)]
struct Config {
    assembly_line: Vec<AssemblyConfig>,
}
#[derive(Debug, Deserialize)]
struct AssemblyConfig {
    #[serde(rename = "type")]
    assembly_type: String,
    parameters: HashMap<String, String>,
}

fn find_assembly(assembly_type: &str) -> Result<SupportedAssemblies> {
    match assembly_type {
        "http" => Ok(Http {}.into()),
        "echo" => Ok(Echo {}.into()),
        "wechat" => Ok(WeChat {}.into()),
        "gist" => Ok(Gist {}.into()),
        _ => Err(anyhow!("Assembly {} is not found.", assembly_type)),
    }
}

fn make_assembly(
    assembly_config: &AssemblyConfig,
    context: &Context,
) -> Result<(SupportedAssemblies, Payload)> {
    let assembly = find_assembly(&assembly_config.assembly_type)?;
    let mut payload: HashMap<&'static str, String> = HashMap::new();
    for key in assembly.parameters() {
        if let Some(value) = assembly_config.parameters.get(*key) {
            payload.insert(key, fulfill(value, &context)?);
        }
    }
    Ok((assembly, Payload::new(payload)))
}

fn main() -> Result<()> {
    let config_path = env::args()
        .nth(1)
        .context("No configuration is provided.")?;

    let config = fs::read_to_string(config_path)?;
    let config: Config = serde_yaml::from_str(&config)?;

    let mut context = Context::new();
    for assembly_config in config.assembly_line.into_iter() {
        let (assembly, payload) = make_assembly(&assembly_config, &context)?;
        let output = assembly.assemble(payload)?;

        context.input = output
            .parameters
            .into_iter()
            .map(|(k, v)| (k.to_string(), v))
            .collect();
    }

    Ok(())
}
