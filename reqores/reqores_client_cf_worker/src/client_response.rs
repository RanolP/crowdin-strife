use reqores::ClientResponse;
use worker::Response;

pub struct CfWorkerClientResponse {
    body: Vec<u8>,
    response: Response,
}

impl CfWorkerClientResponse {
    pub async fn new(mut response: Response) -> worker::Result<Self> {
        Ok(Self {
            body: response.bytes().await?,
            response,
        })
    }
}

impl ClientResponse for CfWorkerClientResponse {
    fn body(&self) -> &[u8] {
        &self.body
    }

    fn header(&self, key: &str) -> Option<String> {
        self.response.headers().get(key).ok().flatten()
    }
}
