pub trait Env {
    fn var(&self, key: &str) -> Option<String>;
}
