use crate::{Context, Input, Inputs, Outputs, Workflow};
use anyhow::Result;
use atom_syndication::Feed;
use chrono::{Duration, Local};
use std::{io::BufReader};

pub struct Atom {}

impl Atom {
    // Input
    const TEXT: &'static str = "text";
    const SCHEDULE_IN_SECS: &'static str = "schedule_in_secs";
    const PARAMS: [&'static str; 2] = [Atom::TEXT, Atom::SCHEDULE_IN_SECS];

    // Output
    const TITLE: &'static str = "title";
    const LINK: &'static str = "link";
    const OUTPUT: [&'static str; 2] = [Atom::TITLE, Atom::LINK];
}

impl Workflow for Atom {
    fn execute(&self, context: &mut Context, input: Inputs) -> Result<()> {
        let text = input.parameter(Atom::TEXT);
        let after = input
            .parameter(Atom::SCHEDULE_IN_SECS)
            .parse()
            .map(|secs| Local::now() - Duration::seconds(secs));

        let next = match context.next() {
            Some(next) => next,
            None => return Ok(()),
        };
        let feed = Feed::read_from(BufReader::new(text.as_bytes()))?;
        for entry in feed.entries() {
            if let Ok(after) = after {
                if entry.updated() < &after {
                    break;
                }
            }
            let mut output = Outputs::new();
            output.insert(Atom::TITLE, entry.title().to_string());
            output.insert(
                Atom::LINK,
                entry
                    .links()
                    .iter()
                    .map(|link| link.href())
                    .collect::<Vec<_>>()
                    .join(","),
            );
            next.execute(context, output)?;
        }

        Ok(())
    }

    fn parameters(&self) -> &'static [&'static str] {
        return &Atom::PARAMS;
    }
    fn outputs(&self) -> &'static [&'static str] {
        return &Atom::OUTPUT;
    }
}
