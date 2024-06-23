use std::{io, path::Path};

use clap::{Arg, ArgAction, Command};

use crate::{
    artifact::Artifact,
    config::ARTIFACT_JSON,
    utils::{read_artifacts, write_artifacts},
};

pub fn do_remove(name: &str, force: &bool) {
    let mut artifacts = read_artifacts(Path::new(ARTIFACT_JSON));
    artifacts.retain(|x| {
        if x.name != name {
            return true;
        }
        if *force {
            return false;
        }
        return !get_user_confirmation(x);
    });
    write_artifacts(&artifacts, Path::new(ARTIFACT_JSON))
}

fn get_user_confirmation(artifact: &Artifact) -> bool {
    println!("Remove {artifact:?}?");
    let mut response = String::new();
    io::stdin()
        .read_line(&mut response)
        .expect("Failed to read the line");
    response.trim().to_lowercase() == "y"
}

pub fn subcmd_remove() -> Command {
    Command::new("remove")
        .about("Remove an artifact from the manifest")
        .arg(
            Arg::new("name")
                .help("The name of the artifact to remove")
                .required(true),
        )
        .arg(
            Arg::new("force")
                .long("force")
                .short('f')
                .help("Skip removal confirmation")
                .action(ArgAction::SetTrue)
                .required(false),
        )
}
