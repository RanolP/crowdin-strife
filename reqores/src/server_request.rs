use serde::Deserialize;

pub trait ServerRequest {
    fn body(&self) -> &[u8];

    fn body_json<'a, T: Deserialize<'a>>(&'a self) -> serde_json::Result<T> {
        serde_json::from_slice(self.body())
    }

    fn header(&self, key: &str) -> Option<String>;
}
