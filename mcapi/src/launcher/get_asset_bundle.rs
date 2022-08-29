use reqores::{ClientRequest, HttpMethod};
use serde::{Deserialize, Serialize};

use super::Version;

pub struct GetAssetBundle<'a> {
    pub version: &'a Version,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetBundle {
    #[serde(rename = "assetIndex")]
    asset_index: AssetIndex,
    downloads: Downloads,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetIndex {
    id: String,
    sha1: String,
    size: u64,
    #[serde(rename = "totalSize")]
    total_size: u64,
    url: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Downloads {
    client: DownloadableFile,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DownloadableFile {
    sha1: String,
    size: u64,
    url: String,
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
