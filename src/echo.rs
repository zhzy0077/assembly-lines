use crate::{Payload, Workflow};
use anyhow::Result;
use std::collections::HashMap;

pub struct Echo {}

impl Echo {
    // Input
    const TEXT: &'static str = "text";
    const PARAMS: [&'static str; 1] = [Echo::TEXT];

    const OUTPUT: [&'static str; 0] = [];
}

impl Workflow for Echo {
    fn execute(&self, payload: Payload) -> Result<Payload> {
        let text = payload.parameter(Echo::TEXT);

        println!("{}", text);

        Ok(Payload::new(HashMap::new()))
    }

    fn parameters(&self) -> &'static [&'static str] {
        return &Echo::PARAMS;
    }
    fn outputs(&self) -> &'static [&'static str] {
        return &Echo::OUTPUT;
    }
}
