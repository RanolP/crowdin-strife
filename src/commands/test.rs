use bot_any::types::MessageOutput;
use bot_any_cal::{Command, CommandSender, Env};

#[derive(Command)]
#[command(name = "sc1", description = "실험용 서브커맨드1입니다.")]
pub struct Subcommand1;

#[derive(Command)]
#[command(name = "sc2", description = "실험용 서브커맨드2입니다.")]
pub struct Subcommand2;

#[derive(Command)]
#[command(name = "test", description = "실험용 명령어입니다.")]
pub enum TestCommand {
    Subcommand1(Subcommand1),
    Subcommand2(Subcommand2),
    #[command(name = "hack", description = "실험용 서브커맨드 hack입니다.")]
    Hack {
        #[argument(name = "arg1", description = "선택적 문자열")]
        arg1: Option<String>,
        #[argument(name = "arg2", description = "선택적 정수")]
        arg2: Option<i64>,
        #[argument(name = "arg3", description = "필수적 실수")]
        arg3: f64,
    },
}

impl TestCommand {
    pub async fn execute(self, sender: CommandSender, env: &impl Env) -> MessageOutput {
        match self {
            TestCommand::Subcommand1(sc) => MessageOutput {
                content: Some("sc1".to_string()),
            },
            TestCommand::Subcommand2(sc) => MessageOutput {
                content: Some("sc2".to_string()),
            },
            TestCommand::Hack { arg1, arg2, arg3 } => MessageOutput {
                content: Some(format!("hack {arg1:?} {arg2:?} {arg3:?}")),
            },
        }
    }
}
