use bot_any::types::{CommandSender, Env, MessageWrite};
use kal::Command;

/// 남은 잔업을 가져올까요?
#[derive(Command)]
#[command(rename = "잔업")]
pub struct WorksLeft;

impl WorksLeft {
    pub async fn execute(self, _sender: CommandSender, _env: &dyn Env) -> MessageWrite {
        MessageWrite::begin()
            .push_str("잔업은 언젠가 완료될 것입니다.".to_string())
            .end()
    }
}
