use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Artifact {
    pub name: String,
    pub url: String,
    pub sha256sum: String,
}
