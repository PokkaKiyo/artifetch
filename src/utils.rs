use std::{
    fs::{read_to_string, OpenOptions},
    io::BufWriter,
    path::Path,
};

use crate::artifact::Artifact;

pub fn read_artifacts(path: &Path) -> Vec<Artifact> {
    let s = read_to_string(path).unwrap();
    serde_json::from_str(&s).expect("Failed to parse")
}

pub fn write_artifacts(artifacts: &Vec<Artifact>, path: &Path) {
    let f = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open(path)
        .expect("Cannot open file");
    let writer = BufWriter::new(f);
    serde_json::to_writer_pretty(writer, artifacts).expect("Failed to write");
}
