use bot_any::types::{CommandSender, Env, MessageOutput};
use kal::command_group;

pub use unknown::handle_unknown;
pub use version::Version;
pub use works_left::WorksLeft;

mod unknown;
mod version;
mod works_left;

command_group! {
    RootCommand {
        WorksLeft,
        Version,
    }
}

impl RootCommand {
    pub async fn execute(self, sender: CommandSender, env: &dyn Env) -> MessageOutput {
        match self {
            RootCommand::WorksLeft(works_left) => works_left.execute(sender, env).await,
            RootCommand::Version(version) => version.execute(sender, env).await,
        }
    }
}
