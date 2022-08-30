use bot_any::types::{CommandSender, Env, MessageOutput};
use kal::Command;

#[derive(Command)]
#[command(name = "잔업", description = "남은 잔업을 가져올까요?")]
pub struct WorksLeft;

impl WorksLeft {
    pub async fn execute(self, _sender: CommandSender, _env: &dyn Env) -> MessageOutput {
        MessageOutput {
            content: Some("잔업은 언젠가 완료될 것입니다.".to_string()),
        }
    }
}
