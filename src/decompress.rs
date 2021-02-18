use crate::{Assembly, Payload};
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

impl Assembly for Decompress {
    fn assemble(&self, payload: Payload) -> Result<Payload> {
        let path = payload.parameter(Decompress::PATH);
        let destination = payload.parameter(Decompress::DESTINATION);

        let tarball = File::open(path)?;
        let tar = GzDecoder::new(tarball);
        let mut archive = Archive::new(tar);
        archive.unpack(destination)?;

        Ok(Payload::new(HashMap::new()))
    }

    fn parameters(&self) -> &'static [&'static str] {
        return &Decompress::PARAMS;
    }
    fn outputs(&self) -> &'static [&'static str] {
        return &Decompress::OUTPUT;
    }
}
