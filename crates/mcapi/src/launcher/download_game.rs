use reqores::{ClientRequest, HttpMethod};

use super::GameFile;

pub struct DownloadGame<'a> {
    pub game_file: &'a GameFile,
}

impl ClientRequest for DownloadGame<'_> {
    type Response = Vec<u8>;

    fn headers(&self) -> Vec<(String, String)> {
        Vec::new()
    }

    fn url(&self) -> String {
        self.game_file.url.clone()
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
