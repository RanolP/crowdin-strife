use reqores::{ClientRequest, HttpMethod};
use serde::{Deserialize, Serialize};

use super::Version;

pub struct GetAssetBundle<'a> {
    pub version: &'a Version,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetBundle {
    #[serde(rename = "assetIndex")]
    pub asset_index: AssetIndexResolution,
    pub downloads: Downloads,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetIndexResolution {
    pub id: String,
    pub sha1: String,
    pub size: u64,
    #[serde(rename = "totalSize")]
    pub total_size: u64,
    pub url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Downloads {
    pub client: DownloadableFile,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DownloadableFile {
    pub sha1: String,
    pub size: u64,
    pub url: String,
}

impl ClientRequest for GetAssetBundle<'_> {
    type Response = AssetBundle;

    fn url(&self) -> String {
        self.version.url.clone()
    }

    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }
}
