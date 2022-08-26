use reqores::{ClientResponse, StatusCode};
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

    fn status(&self) -> StatusCode {
        match self.response.status_code() {
            200 => StatusCode::Ok,
            204 => StatusCode::NoContent,
            400 => StatusCode::BadRequest,
            403 => StatusCode::Forbidden,
            404 => StatusCode::Notfound,
            _ => todo!(),
        }
    }

    fn header(&self, key: &str) -> Option<String> {
        self.response.headers().get(key).ok().flatten()
    }
}
