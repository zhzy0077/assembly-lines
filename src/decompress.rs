use crate::{Context, Input, Inputs, Outputs, Workflow};
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
    fn execute<T>(&self, context: Context, input: Inputs, next: T) -> Result<()>
    where
        T: FnOnce(Context, Outputs) -> Result<()>,
    {
        let path = input.parameter(Decompress::PATH);
        let destination = input.parameter(Decompress::DESTINATION);

        let tarball = File::open(path)?;
        let tar = GzDecoder::new(tarball);
        let mut archive = Archive::new(tar);
        archive.unpack(destination)?;

        next(context, HashMap::new())
    }

    fn parameters(&self) -> &'static [&'static str] {
        return &Decompress::PARAMS;
    }
    fn outputs(&self) -> &'static [&'static str] {
        return &Decompress::OUTPUT;
    }
}
