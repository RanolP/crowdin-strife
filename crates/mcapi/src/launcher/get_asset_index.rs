use std::collections::HashMap;

use reqores::{ClientRequest, HttpMethod};
use serde::{Deserialize, Serialize};

use super::AssetBundle;

pub struct GetAssetIndex<'a> {
    pub bundle: &'a AssetBundle,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AssetIndex {
    pub objects: HashMap<String, Asset>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Asset {
    pub hash: String,
    pub size: u64,
}

impl ClientRequest for GetAssetIndex<'_> {
    type Response = AssetIndex;

    fn url(&self) -> String {
        self.bundle.asset_index.url.clone()
    }

    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }
}
