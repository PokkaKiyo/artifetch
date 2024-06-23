use std::{path::Path, time::Instant};

use clap::Command;
use futures::future::join_all;
use tokio::fs;

use crate::{artifact::Artifact, config::ARTIFACT_JSON, utils::read_artifacts};

#[tokio::main]
pub async fn do_fetch() {
    let artifacts = read_artifacts(Path::new(ARTIFACT_JSON));
    let futures = artifacts.into_iter().map(download_artifact);
    join_all(futures).await;
}

async fn download_artifact(artifact: Artifact) -> Result<(), reqwest::Error> {
    let now = Instant::now();
    println!("Fetching artifact {}", artifact.name);

    let data = reqwest::get(&artifact.url).await?.bytes().await?;
    fs::write(&artifact.name, data)
        .await
        .expect("Failed to write artifact");

    let elapsed = now.elapsed();
    println!(
        "Successfully fetched artifact {}, elapsed: {:.2?}",
        artifact.name, elapsed
    );
    Ok(())
}

pub fn subcmd_fetch() -> Command {
    Command::new("fetch")
        .visible_alias("sync")
        .about("Perform the fetch based on the manifest")
}
