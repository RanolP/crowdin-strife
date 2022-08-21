pub enum StatusCode {
    Ok = 200,
    Forbidden = 403,
    Notfound = 404,
}

pub enum ServerResponsePart {
    Body(Vec<u8>),
    Header(String, String),
    StatusCode(StatusCode),
}

pub struct ServerResponse {
    partial: bool,
    parts: Vec<ServerResponsePart>,
}

pub struct ServerResponseBuilder {
    partial: bool,
    parts: Vec<ServerResponsePart>,
}

impl ServerResponseBuilder {
    pub fn new() -> Self {
        ServerResponseBuilder {
            partial: false,
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
        self.parts.push(ServerResponsePart::Body(body));
        self.partial = false;
        self.build()
    }

    pub fn build(self) -> ServerResponse {
        ServerResponse {
            partial: self.partial,
            parts: self.parts,
        }
    }
}
