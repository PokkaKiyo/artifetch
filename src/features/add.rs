use std::path::Path;

use clap::{Arg, Command};

use crate::{
    artifact::Artifact,
    config::ARTIFACT_JSON,
    utils::{read_artifacts, write_artifacts},
};

pub fn do_add(name: &str, url: &str, sha256sum: &str) {
    let mut artifacts = read_artifacts(Path::new(ARTIFACT_JSON));
    artifacts.push(Artifact {
        name: String::from(name),
        url: String::from(url),
        sha256sum: String::from(sha256sum),
    });
    write_artifacts(&artifacts, Path::new(ARTIFACT_JSON))
}

pub fn subcmd_add() -> Command {
    Command::new("add")
        .about("Add a new artifact to the manifest")
        .arg(
            Arg::new("name")
                .help("The name of the artifact to add")
                .required(true),
        )
        .arg(
            Arg::new("url")
                .help("The url of the artifact to fetch from")
                .required(true),
        )
        .arg(
            Arg::new("sha256sum")
                .help("The expected sha256sum of the artifact")
                .required(true),
        )
}
