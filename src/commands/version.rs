use bot_any::types::{CommandSender, Env, MessageWrite};
use kal::Command;

#[derive(Command)]
#[command(name = "버전", description = "현재 실행 중인 봇의 버전을 가져옵니다.")]
pub struct Version;

impl Version {
    pub async fn execute(self, _sender: CommandSender, env: &dyn Env) -> MessageWrite {
        MessageWrite::begin()
            .push_str(format!(
                "버전 : {}",
                env.var("VERSION")
                    .unwrap_or_else(|| "알 수 없음".to_string())
            ))
            .end()
    }
}
