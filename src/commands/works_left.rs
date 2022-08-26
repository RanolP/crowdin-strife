use bot_any::types::MessageOutput;
use bot_any_cal::{Command, CommandSender, Env};

#[derive(Command)]
#[command(name = "잔업", description = "남은 잔업을 가져올까요?")]
pub struct WorksLeft;

impl WorksLeft {
    pub async fn execute(self, sender: CommandSender, env: &impl Env) -> MessageOutput {
        MessageOutput {
            content: Some("잔업은 언젠가 완료될 것입니다.".to_string()),
        }
    }
}
