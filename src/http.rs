use crate::{Payload, Workflow};
use anyhow::Result;
use reqwest::blocking::{Client, Request};
use std::collections::HashMap;

pub struct Http {}

impl Http {
    // Input
    const URL: &'static str = "url";
    const METHOD: &'static str = "method";
    const PARAMS: [&'static str; 2] = [Http::URL, Http::METHOD];

    // Output
    const STATUS_CODE: &'static str = "status_code";
    const TEXT: &'static str = "text";
    const OUTPUT: [&'static str; 2] = [Http::STATUS_CODE, Http::TEXT];
}

impl Workflow for Http {
    fn execute(&self, payload: Payload) -> Result<Payload> {
        let url = payload.parameter(Http::URL);
        let method = payload.parameter(Http::METHOD);

        let client = Client::new();
        let request = Request::new(method.parse()?, url.parse()?);
        let response = client.execute(request)?;

        let mut result = HashMap::new();
        result.insert(Http::STATUS_CODE, response.status().as_str().to_string());
        result.insert(Http::TEXT, response.text()?);

        Ok(Payload::new(result))
    }

    fn parameters(&self) -> &'static [&'static str] {
        return &Http::PARAMS;
    }
    fn outputs(&self) -> &'static [&'static str] {
        return &Http::OUTPUT;
    }
}
