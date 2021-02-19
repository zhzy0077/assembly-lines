use crate::{Context, Input, Inputs, Workflow};
use anyhow::Result;
use flate2::read::GzDecoder;
use std::{collections::HashMap, fs::File};
use tar::Archive;

pub struct Decompress {}

impl Decompress {
    // Input
    const PATH: &'static str = "path";
    const DESTINATION: &'static str = "destination";
    const PARAMS: [&'static str; 2] = [Decompress::PATH, Decompress::DESTINATION];

    const OUTPUT: [&'static str; 0] = [];
}

impl Workflow for Decompress {
    fn execute(&self, context: &mut Context, input: Inputs) -> Result<()> {
        let path = input.parameter(Decompress::PATH);
        let destination = input.parameter(Decompress::DESTINATION);

        let tarball = File::open(path)?;
        let tar = GzDecoder::new(tarball);
        let mut archive = Archive::new(tar);
        archive.unpack(destination)?;

        if let Some(next) = context.next() {
            next.execute(context, HashMap::new())?;
        }
        Ok(())
    }

    fn parameters(&self) -> &'static [&'static str] {
        return &Decompress::PARAMS;
    }
    fn outputs(&self) -> &'static [&'static str] {
        return &Decompress::OUTPUT;
    }
}
