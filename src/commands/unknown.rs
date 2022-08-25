use bot_any::types::MessageOutput;
use bot_any_cal::{CommandSender};
use worker::Env;

pub async fn handle_unknown(sender: CommandSender, env: &Env) -> worker::Result<MessageOutput> {
    Ok(MessageOutput {
        content: Some("알 수 없는 명령어입니다.".to_string()),
    })
}
