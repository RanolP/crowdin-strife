use kal::{CommandArgumentValue, CommandFragment};

use crate::message::BoxedStructuredMessage;

pub async fn handle_unknown(preflights: &[CommandFragment]) -> BoxedStructuredMessage {
    let mut command = String::new();

    for preflight in preflights {
        match preflight {
            CommandFragment::Select(name) => {
                command.push_str(&format!("select({name}) "));
            }
            CommandFragment::Execute(arguments) => {
                command.push_str("execute(");
                for argument in arguments {
                    let (name, value) = match argument {
                        kal::CommandArgument::Named(name, value) => (name.clone(), value),
                        kal::CommandArgument::Positioned(position, value) => {
                            (position.to_string(), value)
                        }
                    };
                    command.push_str(&format!(
                        "{}={}, ",
                        name,
                        match &value {
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
    Box::new(format!("알 수 없는 명령어입니다: {command}"))
}
