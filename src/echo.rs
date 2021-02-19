use crate::{Context, Input, Inputs, Workflow};
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
    fn execute(&self, context: &mut Context, input: Inputs) -> Result<()> {
        let text = input.parameter(Echo::TEXT);

        println!("{}", text);

        if let Some(next) = context.next() {
            next.execute(context, HashMap::new())?;
        }
        Ok(())
    }

    fn parameters(&self) -> &'static [&'static str] {
        return &Echo::PARAMS;
    }
    fn outputs(&self) -> &'static [&'static str] {
        return &Echo::OUTPUT;
    }
}
