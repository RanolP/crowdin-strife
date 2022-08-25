use bot_any_cal::{Command, CommandSender, Context};

#[derive(Command)]
#[command(name = "sc1")]
pub struct Subcommand1;

#[derive(Command)]
#[command(name = "sc2")]
pub struct Subcommand2;

#[derive(Command)]
#[command(name = "test")]
pub enum TestCommand {
    Subcommand1(Subcommand1),
    Subcommand2(Subcommand2),
    #[command(name = "hack")]
    Hack {
        #[argument(name = "arg1")]
        arg1: String,
        #[argument(name = "arg2")]
        arg2: i64,
        #[argument(name = "arg3")]
        arg3: f64,
    },
    #[command(self)]
    Command {
        #[argument(name = "arg1")]
        arg1: String,
        #[argument(name = "arg2")]
        arg2: i64,
        #[argument(name = "arg3")]
        arg3: f64,
    },
}

impl TestCommand {
    pub async fn execute(self, sender: CommandSender) {
        match self {
            TestCommand::Subcommand1(sc) => {
                // sc.execute(sender, label, context);
            }
            TestCommand::Subcommand2(sc) => {
                // sc.execute(sender, label, context);
            }
            TestCommand::Hack { arg1, arg2, arg3 } => {
                // todo
            }
            TestCommand::Command { arg1, arg2, arg3 } => {
                // sender.reply(format!("{} {} {}", arg1, arg2, arg3));
            }
        }
    }
}
