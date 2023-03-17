use engine::{db::TmDatabase, env::Env};
use kal::command_group;

pub use unknown::handle_unknown;

mod bedrock_e2k2e;
mod dungeons_e2k2e;
mod java_e2k2e;
mod unknown;
mod version;

command_group! {
    pub enum RootCommand {
        Version(version::Version),
        JavaE2K(java_e2k2e::E2K),
        JavaK2E(java_e2k2e::K2E),
        DungeonsE2K(dungeons_e2k2e::E2K),
        DungeonsK2E(dungeons_e2k2e::K2E),
        BedrockE2K(bedrock_e2k2e::E2K),
        BedrockK2E(bedrock_e2k2e::K2E)
    }
}

impl RootCommand {
    pub async fn execute<'a>(
        self,
        env: &'a (impl Env + Sync + Send),
        api: &'a (impl TmDatabase + Sync + Send),
    ) -> eyre::Result<String> {
        match self {
            RootCommand::Version(version) => Ok(version.execute(env).await),
            RootCommand::JavaE2K(java_e2k) => java_e2k.execute(api).await,
            RootCommand::JavaK2E(java_k2e) => java_k2e.execute(api).await,
            RootCommand::DungeonsE2K(dungeons_e2k) => dungeons_e2k.execute(api).await,
            RootCommand::DungeonsK2E(dungeons_k2e) => dungeons_k2e.execute(api).await,
            RootCommand::BedrockE2K(bedrock_e2k) => bedrock_e2k.execute(api).await,
            RootCommand::BedrockK2E(bedrock_k2e) => bedrock_k2e.execute(api).await,
        }
    }
}
