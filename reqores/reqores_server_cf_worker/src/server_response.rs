use reqores::{ServerResponse, ServerResponsePart, StatusCode};
use worker::Response;

pub fn make_response(server_response: ServerResponse) -> worker::Result<Response> {
    let mut response = Response::from_bytes(server_response.body)?;

    for server_response in server_responses {
        for part in server_response.parts {
            match part {
                ServerResponsePart::Header(name, value) => {
                    response.headers_mut().set(&name, &value);
                }
                ServerResponsePart::StatusCode(code) => {
                    response = response.with_status(code as u16);
                }
            }
        }
        if server_response.is_complete {
            break;
        }
    }

    Ok(response)
}
