use crate::{Context, Input, Inputs, Outputs, Workflow};
use anyhow::Result;
use chrono::{DateTime, Duration, Local};
use rss::Channel;
use std::io::BufReader;

pub struct Rss {}

impl Rss {
    // Input
    const TEXT: &'static str = "text";
    const SCHEDULE_IN_SECS: &'static str = "schedule_in_secs";
    const PARAMS: [&'static str; 2] = [Rss::TEXT, Rss::SCHEDULE_IN_SECS];

    // Output
    const TITLE: &'static str = "title";
    const LINK: &'static str = "link";
    const OUTPUT: [&'static str; 2] = [Rss::TITLE, Rss::LINK];
}

impl Workflow for Rss {
    fn execute(&self, context: &mut Context, input: Inputs) -> Result<()> {
        let text = input.parameter(Rss::TEXT);
        let after = input
            .parameter(Rss::SCHEDULE_IN_SECS)
            .parse()
            .map(|secs| Local::now() - Duration::seconds(secs));

        let next = match context.next() {
            Some(next) => next,
            None => return Ok(()),
        };
        let channel = Channel::read_from(BufReader::new(text.as_bytes()))?;
        for item in channel.items() {
            if let (Ok(after), Some(Ok(pub_date))) = (
                &after,
                item.pub_date().map(|s| DateTime::parse_from_rfc2822(s)),
            ) {
                if &pub_date < after {
                    break;
                }
            }
            let mut output = Outputs::new();
            output.insert(
                Rss::TITLE,
                item.title().map(str::to_string).unwrap_or_default(),
            );
            output.insert(
                Rss::LINK,
                item.link().map(str::to_string).unwrap_or_default(),
            );
            next.execute(context, output)?;
        }

        Ok(())
    }

    fn parameters(&self) -> &'static [&'static str] {
        return &Rss::PARAMS;
    }
    fn outputs(&self) -> &'static [&'static str] {
        return &Rss::OUTPUT;
    }
}
