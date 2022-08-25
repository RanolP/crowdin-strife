use bot_any::types::MessageOutput;
use bot_any_cal::{Command, CommandSender, Env};

#[derive(Command)]
#[command(name = "버전")]
pub struct Version;

impl Version {
    pub async fn execute(self, sender: CommandSender, env: &impl Env) -> MessageOutput {
        MessageOutput {
            content: Some(format!(
                "버전 : {}",
                env.var("VERSION").unwrap_or("알 수 없음".to_string())
            )),
        }
    }
}
