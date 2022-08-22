use serde::Serialize;

#[repr(u16)]
pub enum StatusCode {
    Ok = 200,
    BadRequest = 400,
    Forbidden = 403,
    Notfound = 404,
}

pub enum ServerResponsePart {
    Header(String, String),
    StatusCode(StatusCode),
}

pub struct ServerResponse {
    pub parts: Vec<ServerResponsePart>,
    pub body: Option<Vec<u8>>,
}

impl ServerResponse {
    pub fn then(self, other: ServerResponse) -> ServerResponse {
        let mut parts = self.parts;
        parts.extend(other.parts);
        ServerResponse {
            parts,
            body: other.body.or(self.body),
        }
    }
}

pub struct ServerResponseBuilder {
    parts: Vec<ServerResponsePart>,
    body: Option<Vec<u8>>,
}

impl ServerResponseBuilder {
    pub fn new() -> Self {
        ServerResponseBuilder {
            body: None,
            parts: Vec::new(),
        }
    }

    pub fn header(mut self, name: String, value: String) -> Self {
        self.parts.push(ServerResponsePart::Header(name, value));
        self
    }

    pub fn status(mut self, status: StatusCode) -> Self {
        self.parts.push(ServerResponsePart::StatusCode(status));
        self
    }

    pub fn body(mut self, body: Vec<u8>) -> ServerResponse {
        self.body = Some(body);
        self.build()
    }

    pub fn body_json<T: Serialize>(mut self, body: &T) -> serde_json::Result<ServerResponse> {
        Ok(self
            .header(
                "Cotent-Type".to_string(),
                "application/json; charset=UTF-8".to_string(),
            )
            .body(serde_json::to_vec(value)?))
    }

    pub fn build(self) -> ServerResponse {
        ServerResponse {
            parts: self.parts,
            body: self.body,
        }
    }
}
