use bot_any::types::Env;

pub struct CfWorkerEnv(pub worker::Env);

impl Env for CfWorkerEnv {
    fn var(&self, key: &str) -> Option<String> {
        self.0.var(key).ok().map(|s| s.to_string())
    }
}
