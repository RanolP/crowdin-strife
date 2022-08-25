use reqores::{ClientResponse, StatusCode};
use surf::{Response, StatusCode as SurfStatus};

pub struct SurfClientResponse {
    body: Vec<u8>,
    response: Response,
}

impl SurfClientResponse {
    pub async fn new(mut response: Response) -> surf::Result<Self> {
        Ok(Self {
            body: response.body_bytes().await?,
            response,
        })
    }
}

impl ClientResponse for SurfClientResponse {
    fn body(&self) -> &[u8] {
        &self.body
    }

    fn status(&self) -> StatusCode {
        match self.response.status() {
            SurfStatus::Continue => todo!(),
            SurfStatus::SwitchingProtocols => todo!(),
            SurfStatus::EarlyHints => todo!(),
            SurfStatus::Ok => StatusCode::Ok,
            SurfStatus::Created => todo!(),
            SurfStatus::Accepted => todo!(),
            SurfStatus::NonAuthoritativeInformation => todo!(),
            SurfStatus::NoContent => StatusCode::NoContent,
            SurfStatus::ResetContent => todo!(),
            SurfStatus::PartialContent => todo!(),
            SurfStatus::MultiStatus => todo!(),
            SurfStatus::ImUsed => todo!(),
            SurfStatus::MultipleChoice => todo!(),
            SurfStatus::MovedPermanently => todo!(),
            SurfStatus::Found => todo!(),
            SurfStatus::SeeOther => todo!(),
            SurfStatus::NotModified => todo!(),
            SurfStatus::TemporaryRedirect => todo!(),
            SurfStatus::PermanentRedirect => todo!(),
            SurfStatus::BadRequest => StatusCode::BadRequest,
            SurfStatus::Unauthorized => todo!(),
            SurfStatus::PaymentRequired => todo!(),
            SurfStatus::Forbidden => StatusCode::Forbidden,
            SurfStatus::NotFound => StatusCode::Notfound,
            SurfStatus::MethodNotAllowed => todo!(),
            SurfStatus::NotAcceptable => todo!(),
            SurfStatus::ProxyAuthenticationRequired => todo!(),
            SurfStatus::RequestTimeout => todo!(),
            SurfStatus::Conflict => todo!(),
            SurfStatus::Gone => todo!(),
            SurfStatus::LengthRequired => todo!(),
            SurfStatus::PreconditionFailed => todo!(),
            SurfStatus::PayloadTooLarge => todo!(),
            SurfStatus::UriTooLong => todo!(),
            SurfStatus::UnsupportedMediaType => todo!(),
            SurfStatus::RequestedRangeNotSatisfiable => todo!(),
            SurfStatus::ExpectationFailed => todo!(),
            SurfStatus::ImATeapot => todo!(),
            SurfStatus::MisdirectedRequest => todo!(),
            SurfStatus::UnprocessableEntity => todo!(),
            SurfStatus::Locked => todo!(),
            SurfStatus::FailedDependency => todo!(),
            SurfStatus::TooEarly => todo!(),
            SurfStatus::UpgradeRequired => todo!(),
            SurfStatus::PreconditionRequired => todo!(),
            SurfStatus::TooManyRequests => todo!(),
            SurfStatus::RequestHeaderFieldsTooLarge => todo!(),
            SurfStatus::UnavailableForLegalReasons => todo!(),
            SurfStatus::InternalServerError => todo!(),
            SurfStatus::NotImplemented => todo!(),
            SurfStatus::BadGateway => todo!(),
            SurfStatus::ServiceUnavailable => todo!(),
            SurfStatus::GatewayTimeout => todo!(),
            SurfStatus::HttpVersionNotSupported => todo!(),
            SurfStatus::VariantAlsoNegotiates => todo!(),
            SurfStatus::InsufficientStorage => todo!(),
            SurfStatus::LoopDetected => todo!(),
            SurfStatus::NotExtended => todo!(),
            SurfStatus::NetworkAuthenticationRequired => todo!(),
        }
    }

    fn header(&self, key: &str) -> Option<String> {
        self.response.header(key).map(|v| v.to_string())
    }
}
