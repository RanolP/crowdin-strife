use bot_any::types::{CommandSender, Env, MessageWrite};
use kal::command_group;

pub use unknown::handle_unknown;

use crate::file_reader::AssetStore;

// mod bedrock_e2k2e;
mod dungeons_e2k2e;
mod java_e2k2e;
mod unknown;
mod version;
mod works_left;

command_group! {
    pub enum RootCommand {
        WorksLeft(works_left::WorksLeft),
        Version(version::Version),
        JavaE2K(java_e2k2e::E2K),
        JavaK2E(java_e2k2e::K2E),
        DungeonsE2K(dungeons_e2k2e::E2K),
        DungeonsK2E(dungeons_e2k2e::K2E)
        // BedrockE2K(bedrock_e2k2e::E2K),
        // BedrockK2E(bedrock_e2k2e::K2E)
    }
}

impl RootCommand {
    pub async fn execute<'a>(
        self,
        sender: CommandSender,
        env: &'a dyn Env,
        asset_store: &'a AssetStore<'a>,
    ) -> eyre::Result<MessageWrite> {
        match self {
            RootCommand::WorksLeft(works_left) => Ok(works_left.execute(sender, env).await),
            RootCommand::Version(version) => Ok(version.execute(sender, env).await),
            RootCommand::JavaE2K(java_e2k) => java_e2k.execute(asset_store).await,
            RootCommand::JavaK2E(java_k2e) => java_k2e.execute(asset_store).await,
            RootCommand::DungeonsE2K(dungeons_e2k) => dungeons_e2k.execute(asset_store).await,
            RootCommand::DungeonsK2E(dungeons_k2e) => dungeons_k2e.execute(asset_store).await,
            // RootCommand::BedrockE2K(bedrock_e2k) => bedrock_e2k.execute(asset_store).await,
            // RootCommand::BedrockK2E(bedrock_k2e) => bedrock_k2e.execute(asset_store).await,
        }
    }
}
