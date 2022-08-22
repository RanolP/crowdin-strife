use client_response::CfWorkerClientResponse;
use reqores::{ClientRequest, HttpMethod};
use worker::{wasm_bindgen::JsValue, Fetch, Headers, Method, Request, RequestInit};

mod client_response;

pub struct CfWorkerClient;

impl CfWorkerClient {
    pub async fn call<Req: ClientRequest>(
        &self,
        client_request: Req,
    ) -> Result<Req::Response, worker::Error> {
        let mut headers = Headers::new();
        headers.set("Content-Type", "application/json; charset=UTF-8")?;
        for (k, v) in client_request.headers() {
            headers.set(&k, &v)?;
        }

        let mut request_init = RequestInit::new();
        request_init
            .with_method(match client_request.method() {
                &HttpMethod::Get => Method::Get,
                &HttpMethod::Post => Method::Post,
                &HttpMethod::Delete => Method::Delete,
            })
            .with_headers(headers)
            .with_body(client_request.body().map(|s| JsValue::from_str(&s)));

        let request = Fetch::Request(Request::new_with_init(
            &client_request.url(),
            &request_init,
        )?);
        let response = request.send().await?;
        let client_response = CfWorkerClientResponse::new(response).await?;

        let result = client_request
            .deserialize(&client_response)
            .map_err(worker::Error::RustError)?;

        Ok(result)
    }
}
