use reqores::{ClientRequest, HttpMethod};

pub struct SurfClient;

impl SurfClient {
    pub async fn call<Req: ClientRequest>(
        &self,
        client_request: Req,
    ) -> Result<Req::Response, surf::Error> {
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
        let mut response = request.send().await?;

        if let Some(header_processor) = client_request.header_processor() {
            for (name, value) in response.iter() {
                if let Some(v) = header_processor(&name.as_str(), &value.as_str()) {
                    return Ok(v);
                }
            }
        }

        response.body_json().await
    }
}
