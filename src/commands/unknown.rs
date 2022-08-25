use bot_any::types::MessageOutput;
use bot_any_cal::{CommandArgumentValue, CommandPreflight, CommandSender, Env};

pub async fn handle_unknown(
    sender: CommandSender,
    preflights: &[CommandPreflight],
    env: &impl Env,
) -> MessageOutput {
    let mut command = String::new();

    for preflight in preflights {
        match preflight {
            CommandPreflight::Select(name) => {
                command.push_str(&format!("select({name}) "));
            }
            CommandPreflight::Execute(arguments) => {
                command.push_str("execute(");
                for argument in arguments {
                    command.push_str(&format!(
                        "{}={}, ",
                        argument.name,
                        match &argument.value {
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
                command.push_str(")");
            }
        }
    }

    MessageOutput {
        content: Some(format!("알 수 없는 명령어입니다: {command}")),
    }
}
