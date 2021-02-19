use crate::{Context, Input, Inputs, Outputs, Workflow};
use anyhow::Result;
use std::{
    collections::HashMap,
    process::{Command as StdCommand, Stdio},
};

pub struct Command {}

impl Command {
    // Input
    const PROGRAM: &'static str = "program";
    const DAEMON: &'static str = "daemon";
    const INHERIT_IO: &'static str = "inherit_io";
    const PARAMS: [&'static str; 3] = [Command::PROGRAM, Command::DAEMON, Command::INHERIT_IO];

    const OUTPUT: [&'static str; 0] = [];
}

impl Workflow for Command {
    fn execute<T>(&self, context: Context, input: Inputs, next: T) -> Result<()>
    where
        T: FnOnce(Context, Outputs) -> Result<()>,
    {
        let program = input.parameter(Command::PROGRAM);
        let daemon: bool = input.parameter(Command::DAEMON).parse().unwrap_or(false);
        let inherit_io: bool = input
            .parameter(Command::INHERIT_IO)
            .parse()
            .unwrap_or(false);

        let mut command = StdCommand::new(program);
        if !inherit_io {
            command.stdout(Stdio::null());
            command.stderr(Stdio::null());
        }
        let mut handle = command.spawn()?;
        if !daemon {
            handle.wait()?;
        }

        next(context, HashMap::new())
    }

    fn parameters(&self) -> &'static [&'static str] {
        return &Command::PARAMS;
    }
    fn outputs(&self) -> &'static [&'static str] {
        return &Command::OUTPUT;
    }
}
