use reqores::ClientResponse;
use surf::Response;

pub struct SurfClientResponse {
    body: Vec<u8>,
    response: Response,
}

impl SurfClientResponse {
    pub async fn new(mut response: Response) -> surf::Result<Self> {
        Ok(Self {
            body: response.body_bytes().await?,
            response,
        })
    }
}

impl ClientResponse for SurfClientResponse {
    fn body(&self) -> &[u8] {
        &self.body
    }

    fn header(&self, key: &str) -> Option<String> {
        self.response.header(key).map(|v| v.to_string())
    }
}
