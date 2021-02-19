use crate::{Context, Input, Inputs, Outputs, Workflow};
use anyhow::Result;
use std::collections::HashMap;

pub struct Atom {}

impl Atom {
    // Input
    const URL: &'static str = "url";
    const SCHEDULE_IN_MINUTE: &'static str = "schedule_in_minute";
    const PARAMS: [&'static str; 2] = [Atom::URL, Atom::SCHEDULE_IN_MINUTE];

    const OUTPUT: [&'static str; 0] = [];
}

impl Workflow for Atom {
    fn execute<T>(&self, context: Context, input: Inputs, next: T) -> Result<()>
    where
        T: FnOnce(Context, Outputs) -> Result<()>,
    {
        let text = input.parameter(Atom::URL);

        println!("{}", text);

        next(context, HashMap::new())
    }

    fn parameters(&self) -> &'static [&'static str] {
        return &Atom::PARAMS;
    }
    fn outputs(&self) -> &'static [&'static str] {
        return &Atom::OUTPUT;
    }
}
