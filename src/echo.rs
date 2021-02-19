use crate::{Context, Input, Inputs, Outputs, Workflow};
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
    fn execute<T>(&self, context: Context, input: Inputs, next: T) -> Result<()>
    where
        T: FnOnce(Context, Outputs) -> Result<()>,
    {
        let text = input.parameter(Echo::TEXT);

        println!("{}", text);

        next(context, HashMap::new())
    }

    fn parameters(&self) -> &'static [&'static str] {
        return &Echo::PARAMS;
    }
    fn outputs(&self) -> &'static [&'static str] {
        return &Echo::OUTPUT;
    }
}
