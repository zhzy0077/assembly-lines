use crate::{Context, Input, Inputs, Workflow};
use anyhow::Result;
use std::io::Write;
use std::{collections::HashMap, fs::File};

pub struct Save {}

impl Save {
    // Input
    const TEXT: &'static str = "text";
    const DESTINATION: &'static str = "destination";
    const PARAMS: [&'static str; 2] = [Save::TEXT, Save::DESTINATION];

    const OUTPUT: [&'static str; 0] = [];
}

impl Workflow for Save {
    fn execute(&self, context: &mut Context, input: Inputs) -> Result<()> {
        let text = input.parameter(Save::TEXT);
        let destination = input.parameter(Save::DESTINATION);

        let mut file = File::create(destination)?;
        file.write_all(text.as_bytes())?;

        if let Some(next) = context.next() {
            next.execute(context, HashMap::new())?;
        }
        Ok(())
    }

    fn parameters(&self) -> &'static [&'static str] {
        return &Save::PARAMS;
    }
    fn outputs(&self) -> &'static [&'static str] {
        return &Save::OUTPUT;
    }
}
