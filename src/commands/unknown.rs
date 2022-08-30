use bot_any::types::{CommandSender, Env, MessageOutput};
use kal::{CommandArgumentValue, CommandFragment};

pub async fn handle_unknown(
    _sender: CommandSender,
    preflights: &[CommandFragment],
    _env: &impl Env,
) -> MessageOutput {
    let mut command = String::new();

    for preflight in preflights {
        match preflight {
            CommandFragment::Select(name) => {
                command.push_str(&format!("select({name}) "));
            }
            CommandFragment::Execute(arguments) => {
                command.push_str("execute(");
                for argument in arguments {
                    command.push_str(&format!(
                        "{}={}, ",
                        argument.0,
                        match &argument.1 {
                            CommandArgumentValue::String(s) => format!("str({s})"),
                            CommandArgumentValue::I64(i) => format!("i64({i})"),
                            CommandArgumentValue::F64(f) => format!("f64({f})"),
                        }
                    ));
                }
                if !arguments.is_empty() {
                    command.pop();
                    command.pop();
                }
                command.push(')');
            }
        }
    }

    MessageOutput {
        content: Some(format!("알 수 없는 명령어입니다: {command}")),
    }
}
