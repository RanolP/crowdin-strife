use reqores::{ClientRequest, HttpMethod};
use worker::{wasm_bindgen::JsValue, Fetch, Headers, Method, Request, RequestInit};

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
        let mut response = request.send().await?;

        if let Some(header_processor) = client_request.header_processor() {
            for (k, v) in response.headers() {
                if let Some(v) = header_processor(&k, &v) {
                    return Ok(v);
                }
            }
        }

        response.json().await
    }
}
