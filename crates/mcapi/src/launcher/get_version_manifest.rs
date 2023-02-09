use reqores::{ClientRequest, HttpMethod};
use serde::{Deserialize, Serialize};

pub struct GetVersionManifest;

#[derive(Serialize, Deserialize)]
pub struct VersionManifest {
    pub latest: LatestVersionIds,
    pub versions: Vec<Version>,
}

#[derive(Serialize, Deserialize)]
pub struct LatestVersionIds {
    pub release: String,
    pub snapshot: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Version {
    pub id: String,

    #[serde(rename = "type")]
    pub kind: VersionKind,

    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum VersionKind {
    #[serde(rename = "release")]
    Release,
    #[serde(rename = "snapshot")]
    Snapshot,
    #[serde(rename = "old_beta")]
    OldBeta,
    #[serde(rename = "old_alpha")]
    OldAlpha,
}

impl ClientRequest for GetVersionManifest {
    type Response = VersionManifest;

    fn url(&self) -> String {
        "https://launchermeta.mojang.com/mc/game/version_manifest.json".to_string()
    }

    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }
}
