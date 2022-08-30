use reqores::{ClientRequest, HttpMethod};

use super::Asset;

pub struct DownloadAsset<'a> {
    pub asset: &'a Asset,
}

impl ClientRequest for DownloadAsset<'_> {
    type Response = Vec<u8>;

    fn url(&self) -> String {
        format!(
            "http://resources.download.minecraft.net/{}/{}",
            &self.asset.hash[..2],
            self.asset.hash
        )
    }

    fn method(&self) -> HttpMethod {
        HttpMethod::Get
    }

    fn deserialize(
        &self,
        response: &dyn reqores::ClientResponse,
    ) -> Result<Self::Response, String> {
        Ok(response.body().to_vec())
    }
}
