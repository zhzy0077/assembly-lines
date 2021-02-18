use crate::{Assembly, Payload, USER_AGENT};
use anyhow::Result;
use reqwest::blocking::Client;
use reqwest::blocking::Response;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use strum::EnumString;

pub struct Gist {}

#[derive(Debug, EnumString)]
enum GistAction {
    GET,
    UPDATE,
}

#[derive(Debug, Serialize, Deserialize)]
struct GistPayload<'a> {
    #[serde(borrow)]
    files: HashMap<&'a str, GistFile<'a>>,
}

#[derive(Debug, Serialize, Deserialize)]
struct GistFile<'a> {
    content: &'a str,
}

impl Gist {
    pub const TEXT: &'static str = "text";

    // Input
    const ACTION: &'static str = "action";
    const GIST_ID: &'static str = "gist_id";
    const ACCESS_TOKEN: &'static str = "access_token";
    const FILE_NAME: &'static str = "file_name";
    const PARAMS: [&'static str; 5] = [
        Gist::ACTION,
        Gist::GIST_ID,
        Gist::ACCESS_TOKEN,
        Gist::FILE_NAME,
        Gist::TEXT,
    ];

    // Output
    const STATUS_CODE: &'static str = "status_code";
    const OUTPUT: [&'static str; 2] = [Gist::STATUS_CODE, Gist::TEXT];

    fn update(gist_id: &str, access_token: &str, file_name: &str, text: &str) -> Result<Response> {
        let url = format!("https://api.github.com/gists/{}", gist_id);

        let mut files = HashMap::new();
        files.insert(file_name, GistFile { content: text });

        let gist_payload = GistPayload { files };
        let client = Client::new();

        Ok(client
            .patch(&url)
            .header("Authorization", format!("token {}", access_token))
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", USER_AGENT)
            .json(&gist_payload)
            .send()?)
    }

    fn get(gist_id: &str) -> Result<Response> {
        let url = format!("https://api.github.com/gists/{}", gist_id);

        let client = Client::new();
        Ok(client
            .get(&url)
            .header("Accept", "application/vnd.github.v3+json")
            .header("User-Agent", USER_AGENT)
            .send()?)
    }
}

impl Assembly for Gist {
    fn assemble(&self, payload: Payload) -> Result<Payload> {
        let action: GistAction = payload.parameter(Gist::ACTION).to_uppercase().parse()?;
        let gist_id = payload.parameter(Gist::GIST_ID);
        let access_token = payload.parameter(Gist::ACCESS_TOKEN);
        let file_name = payload.parameter(Gist::FILE_NAME);
        let text = payload.parameter(Gist::TEXT);

        let response = match action {
            GistAction::GET => Gist::get(gist_id),
            GistAction::UPDATE => Gist::update(gist_id, access_token, file_name, text),
        }?;

        let mut result = HashMap::new();
        result.insert(Gist::STATUS_CODE, response.status().as_str().to_string());

        let content: String = response.text()?;
        let resp: GistPayload = serde_json::from_str(&content)?;

        result.insert(Gist::TEXT, resp.files[file_name].content.to_string());

        Ok(Payload::new(result))
    }

    fn parameters(&self) -> &'static [&'static str] {
        return &Gist::PARAMS;
    }
    fn outputs(&self) -> &'static [&'static str] {
        return &Gist::OUTPUT;
    }
}
