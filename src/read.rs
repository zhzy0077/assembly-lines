use crate::{Context, Input, Inputs, Outputs, Workflow};
use anyhow::Result;
use std::{fs::File, io::Read as _};

pub struct Read {}

impl Read {
    // Input
    const PATH: &'static str = "path";
    const PARAMS: [&'static str; 1] = [Read::PATH];

    const TEXT: &'static str = "text";
    const OUTPUT: [&'static str; 1] = [Read::TEXT];
}

impl Workflow for Read {
    fn execute(&self, context: &mut Context, input: Inputs) -> Result<()> {
        let path = input.parameter(Read::PATH);

        let mut text = String::new();
        File::open(path)?.read_to_string(&mut text)?;

        let mut output = Outputs::new();
        output.insert(Read::TEXT, text);
        if let Some(next) = context.next() {
            next.execute(context, output)?;
        }
        Ok(())
    }

    fn parameters(&self) -> &'static [&'static str] {
        return &Read::PARAMS;
    }
    fn outputs(&self) -> &'static [&'static str] {
        return &Read::OUTPUT;
    }
}
