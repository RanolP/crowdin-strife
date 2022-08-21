use reqores::ServerRequest;
use worker::Request;

pub struct CfWorkerServerRequest {
    request: Request,
    body: Vec<u8>,
}

impl CfWorkerServerRequest {
    pub async fn new(mut request: Request) -> worker::Result<Self> {
        let body = request.bytes().await?;
        Ok(Self { request, body })
    }
}

impl ServerRequest for CfWorkerServerRequest {
    fn body(&self) -> &[u8] {
        &self.body
    }

    fn header(&self, key: &str) -> Option<String> {
        self.request.headers().get(key).ok().flatten()
    }
}
