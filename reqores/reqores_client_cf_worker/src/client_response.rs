use reqores::{ClientResponse, HttpStatusCode};
use worker::Response;

pub struct CfWorkerClientResponse {
    body: Vec<u8>,
    response: Response,
    status: HttpStatusCode,
}

impl CfWorkerClientResponse {
    pub async fn new(mut response: Response) -> worker::Result<Self> {
        let body = response.bytes().await?;
        let status = HttpStatusCode::try_from(response.status_code())
            .map_err(|e| worker::Error::RustError(e.to_string()))?;
        Ok(Self {
            body,
            response,
            status,
        })
    }
}

impl ClientResponse for CfWorkerClientResponse {
    fn body(&self) -> &[u8] {
        &self.body
    }

    fn status(&self) -> HttpStatusCode {
        self.status.clone()
    }

    fn header(&self, key: &str) -> Option<String> {
        self.response.headers().get(key).ok().flatten()
    }
}
