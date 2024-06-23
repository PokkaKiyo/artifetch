use std::path::Path;

use clap::Command;

use crate::{config::ARTIFACT_JSON, utils::read_artifacts};

pub fn do_show() {
    let artifacts = read_artifacts(Path::new(ARTIFACT_JSON));
    println!("{artifacts:?}")
}

pub fn subcmd_show() -> Command {
    Command::new("show")
        .visible_alias("list")
        .about("Display the current manifest")
}
