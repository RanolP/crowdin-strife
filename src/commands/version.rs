use bot_any::types::{CommandSender, Env, MessageOutput};
use kal::Command;

#[derive(Command)]
#[command(name = "버전", description = "현재 실행 중인 봇의 버전을 가져옵니다.")]
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
