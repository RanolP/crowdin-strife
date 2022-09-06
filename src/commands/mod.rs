use bot_any::types::{CommandSender, Env, MessageWrite};
use kal::command_group;

pub use unknown::handle_unknown;

mod e2k2e;
mod unknown;
mod version;
mod works_left;

command_group! {
    pub enum RootCommand {
        WorksLeft(works_left::WorksLeft),
        Version(version::Version),
        E2K(e2k2e::E2K),
        K2E(e2k2e::K2E)
    }
}

impl RootCommand {
    pub async fn execute(self, sender: CommandSender, env: &dyn Env) -> eyre::Result<MessageWrite> {
        match self {
            RootCommand::WorksLeft(works_left) => Ok(works_left.execute(sender, env).await),
            RootCommand::Version(version) => Ok(version.execute(sender, env).await),
            RootCommand::E2K(e2k) => e2k.execute().await,
            RootCommand::K2E(k2e) => k2e.execute().await,
        }
    }
}
