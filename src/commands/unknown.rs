use bot_any::types::MessageOutput;
use bot_any_cal::{CommandSender, Env};

pub async fn handle_unknown(sender: CommandSender, env: &impl Env) -> MessageOutput {
    MessageOutput {
        content: Some("알 수 없는 명령어입니다.".to_string()),
    }
}
