#![cfg(not(target_arch = "wasm32"))]

use bot_any_cal::Command;
use bot_any_platform_discord::sys::{
    commands::{DeleteCommand, ListCommands, UpdateCommand},
    types::{
        ApplicationCommand, ApplicationCommandKind, ApplicationCommandOption,
        ApplicationCommandOptionKind, Snowflake,
    },
};
use crowdin_strife::commands::{RootCommand, TestCommand, Version, WorksLeft};
use reqores_client_surf::SurfClient;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenvy::dotenv().ok();
    color_eyre::install().ok();

    let discord_application_id = dotenvy::var("DISCORD_APPLICATION_ID")?;
    let discord_token = dotenvy::var("DISCORD_TOKEN")?;
    let guild_id = Some("898200418146988072".to_string());

    let guild_id = guild_id.map(Snowflake);

    let client = SurfClient;

    let old_commands = client
        .call(ListCommands {
            application_id: &discord_application_id,
            token: &discord_token,
            guild_id: guild_id.clone(),
        })
        .await
        .map_err(|e| eyre::eyre!("{}", e))?;

    for command in old_commands {
        if RootCommand::contains(&command.name) {
            continue;
        }
        println!("Deleting {}", command.name);
        client
            .call(DeleteCommand {
                application_id: &discord_application_id,
                token: &discord_token,
                command_id: command.id.unwrap(),
            })
            .await
            .map_err(|e| eyre::eyre!("{}", e))?;
    }

    println!();

    let result = client
        .call(UpdateCommand {
            application_id: &discord_application_id,
            token: &discord_token,
            // guild_id: None,
            guild_id: guild_id.clone(),
            command: ApplicationCommand {
                application_id: Some(Snowflake(discord_application_id.clone())),
                description: Some("아직 기능이 없어요 ㅠ".to_string()),
                ..ApplicationCommand::try_from(WorksLeft::spec())?
            },
        })
        .await
        .map_err(|e| eyre::eyre!("{}", e))?;
    println!("Registering /{}", result.name);

    let result = client
        .call(UpdateCommand {
            application_id: &discord_application_id,
            token: &discord_token,
            // guild_id: None,
            guild_id: guild_id.clone(),
            command: ApplicationCommand {
                application_id: Some(Snowflake(discord_application_id.clone())),
                description: Some("버전 정보를 가져옵니다.".to_string()),
                ..ApplicationCommand::try_from(Version::spec())?
            },
        })
        .await
        .map_err(|e| eyre::eyre!("{}", e))?;
    println!("Registering /{}", result.name);

    let result = client
        .call(UpdateCommand {
            application_id: &discord_application_id,
            token: &discord_token,
            // guild_id: None,
            guild_id: guild_id.clone(),
            command: ApplicationCommand::try_from(TestCommand::spec())?,
        })
        .await
        .map_err(|e| eyre::eyre!("{}", e))?;
    println!("Registering /{}", result.name);

    Ok(())
}
