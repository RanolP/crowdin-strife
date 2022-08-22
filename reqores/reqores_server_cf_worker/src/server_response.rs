use reqores::{ServerResponse, ServerResponsePart};
use worker::Response;

pub fn make_response(server_response: ServerResponse) -> worker::Result<Response> {
    let mut response = Response::from_bytes(server_response.body.unwrap_or_default())?;

    for part in server_response.parts {
        match part {
            ServerResponsePart::Header(name, value) => {
                response.headers_mut().set(&name, &value)?;
            }
            ServerResponsePart::StatusCode(code) => {
                response = response.with_status(code as u16);
            }
        }
    }

    Ok(response)
}
