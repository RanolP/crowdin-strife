use client_response::SurfClientResponse;
use reqores::{ClientRequest, HttpMethod};

mod client_response;

pub struct SurfClient;

impl SurfClient {
    pub async fn call<Req: ClientRequest>(
        &self,
        client_request: Req,
    ) -> surf::Result<Req::Response> {
        let mut request = match client_request.method() {
            &HttpMethod::Get => surf::get(&client_request.url()),
            &HttpMethod::Post => surf::post(&client_request.url()),
            &HttpMethod::Delete => surf::delete(&client_request.url()),
        };
        request = request.header("Content-Type", "application/json; charset=UTF-8");
        for (k, v) in client_request.headers() {
            request = request.header(&*k, v);
        }

        if let Some(body) = client_request.body() {
            request = request.body(body);
        }

        let response = request.send().await?;
        let client_response = SurfClientResponse::new(response).await?;

        client_request
            .deserialize(&client_response)
            .map_err(|s| surf::Error::from_str(500, s))
    }
}
