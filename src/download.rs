use crate::{Payload, Workflow};
use anyhow::Result;
use reqwest::blocking::Client;
use std::{collections::HashMap, fs::File, io};

pub struct Download {}

impl Download {
    // Input
    const URL: &'static str = "url";
    const DESTINATION: &'static str = "destination";
    const PARAMS: [&'static str; 2] = [Download::URL, Download::DESTINATION];

    const OUTPUT: [&'static str; 0] = [];
}

impl Workflow for Download {
    fn execute(&self, payload: Payload) -> Result<Payload> {
        let url = payload.parameter(Download::URL);
        let destination = payload.parameter(Download::DESTINATION);

        let mut file = File::create(destination)?;

        let client = Client::new();
        let mut response = client.get(url).send()?;
        io::copy(&mut response, &mut file)?;

        Ok(Payload::new(HashMap::new()))
    }

    fn parameters(&self) -> &'static [&'static str] {
        return &Download::PARAMS;
    }
    fn outputs(&self) -> &'static [&'static str] {
        return &Download::OUTPUT;
    }
}
