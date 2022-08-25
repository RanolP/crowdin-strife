use thiserror::Error;

pub enum CommandPreflight {
    Select(String),
    Execute(Vec<CommandArgument>),
}

pub struct CommandArgument {
    pub name: String,
    pub value: CommandArgumentValue,
}

#[derive(Clone)]
pub enum CommandArgumentValue {
    String(String),
    I64(i64),
    F64(f64),
}

#[derive(Debug, Error)]
#[error("Expected {expected} but actual type is {actual}")]
pub struct CommandArgumentValueTypeMismatchError {
    pub expected: String,
    pub actual: String,
}

impl TryFrom<CommandArgumentValue> for String {
    type Error = CommandArgumentValueTypeMismatchError;

    fn try_from(value: CommandArgumentValue) -> Result<Self, Self::Error> {
        match value {
            CommandArgumentValue::String(v) => Ok(v),
            CommandArgumentValue::I64(_) => Err(CommandArgumentValueTypeMismatchError {
                expected: "String".to_string(),
                actual: "i64".to_string(),
            }),
            CommandArgumentValue::F64(_) => Err(CommandArgumentValueTypeMismatchError {
                expected: "String".to_string(),
                actual: "f64".to_string(),
            }),
        }
    }
}
impl TryFrom<CommandArgumentValue> for i64 {
    type Error = CommandArgumentValueTypeMismatchError;

    fn try_from(value: CommandArgumentValue) -> Result<Self, Self::Error> {
        match value {
            CommandArgumentValue::String(_) => Err(CommandArgumentValueTypeMismatchError {
                expected: "i64".to_string(),
                actual: "Stringn".to_string(),
            }),
            CommandArgumentValue::I64(v) => Ok(v),
            CommandArgumentValue::F64(_) => Err(CommandArgumentValueTypeMismatchError {
                expected: "i64".to_string(),
                actual: "f64".to_string(),
            }),
        }
    }
}
impl TryFrom<CommandArgumentValue> for f64 {
    type Error = CommandArgumentValueTypeMismatchError;

    fn try_from(value: CommandArgumentValue) -> Result<Self, Self::Error> {
        match value {
            CommandArgumentValue::String(_) => Err(CommandArgumentValueTypeMismatchError {
                expected: "f64".to_string(),
                actual: "String".to_string(),
            }),
            CommandArgumentValue::I64(_) => Err(CommandArgumentValueTypeMismatchError {
                expected: "f64".to_string(),
                actual: "i64".to_string(),
            }),
            CommandArgumentValue::F64(v) => Ok(v),
        }
    }
}
