use crate::{Context, Input, Inputs, Outputs, Workflow};

use anyhow::Result;
use reqwest::blocking::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Deserialize)]
pub struct WeChatAccessToken {
    #[serde(rename = "errcode")]
    error_code: u64,
    #[serde(rename = "errmsg")]
    error_message: String,
    access_token: String,
    expires_in: u64,
}

#[derive(Debug, Serialize)]
struct WeChatMessage<'a> {
    #[serde(rename = "touser")]
    to_user: &'a str,
    #[serde(rename = "toparty")]
    to_party: Option<&'a str>,
    #[serde(rename = "agentid")]
    agent_id: i64,
    #[serde(rename = "msgtype")]
    message_type: &'a str,
    text: WeChatMessageText<'a>,
    #[serde(serialize_with = "crate::util::bool_to_int")]
    enable_duplicate_check: bool,
    duplicate_check_interval: u64,
}

#[derive(Debug, Serialize)]
struct WeChatMessageText<'a> {
    content: &'a str,
}

#[derive(Debug, Deserialize)]
struct WeChatSendResponse {
    #[serde(rename = "errcode")]
    error_code: u64,
    #[serde(rename = "errmsg")]
    error_message: String,
}

pub struct WeChat {}

impl WeChat {
    // Input
    const CORP_ID: &'static str = "corp_id";
    const CORP_SECRET: &'static str = "secret";
    const AGENT_ID: &'static str = "agent_id";
    const TEXT: &'static str = "text";
    const PARAMS: [&'static str; 4] = [
        WeChat::CORP_ID,
        WeChat::CORP_SECRET,
        WeChat::AGENT_ID,
        WeChat::TEXT,
    ];

    // Output
    const ERROR_CODE: &'static str = "error_code";
    const OUTPUT: [&'static str; 0] = [];
}

impl Workflow for WeChat {
    fn execute<T>(&self, context: Context, input: Inputs, next: T) -> Result<()>
    where
        T: FnOnce(Context, Outputs) -> Result<()>,
    {
        let corp_id = input.parameter(WeChat::CORP_ID);
        let secret = input.parameter(WeChat::CORP_SECRET);
        let agent_id = input.parameter(WeChat::AGENT_ID).parse()?;
        let text = input.parameter(WeChat::TEXT);

        let client = Client::new();

        let url = format!(
            "https://qyapi.weixin.qq.com/cgi-bin/gettoken?corpid={}&corpsecret={}",
            corp_id, secret
        );

        let response = client.get(&url).send()?;
        let token: WeChatAccessToken = response.json()?;

        let message = WeChatMessage {
            to_user: "@all",
            to_party: None,
            agent_id: agent_id,
            message_type: "text",
            text: WeChatMessageText { content: text },
            enable_duplicate_check: false,
            duplicate_check_interval: 0,
        };

        let url = format!(
            "https://qyapi.weixin.qq.com/cgi-bin/message/send?access_token={}",
            token.access_token
        );
        let response: WeChatSendResponse = client.post(&url).json(&message).send()?.json()?;

        let mut result = HashMap::new();
        result.insert(WeChat::ERROR_CODE, response.error_code.to_string());
        next(context, result)
    }

    fn parameters(&self) -> &'static [&'static str] {
        return &WeChat::PARAMS;
    }
    fn outputs(&self) -> &'static [&'static str] {
        return &WeChat::OUTPUT;
    }
}
