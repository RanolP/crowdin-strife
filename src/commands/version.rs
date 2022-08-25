use bot_any::types::MessageOutput;
use bot_any_cal::{Command, CommandSender};
use worker::Env;

#[derive(Command)]
#[command(name = "버전")]
pub struct Version;

impl Version {
    pub async fn execute(self, sender: CommandSender, env: &Env) -> worker::Result<MessageOutput> {
        Ok(MessageOutput {
            content: Some(format!(
                "버전 : {}",
                env.var("VERSION")
                    .map(|s| s.to_string())
                    .unwrap_or("알 수 없음".to_string())
            )),
        })
    }
}
