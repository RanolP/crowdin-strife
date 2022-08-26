#![cfg(not(target_arch = "wasm32"))]

use std::time::Duration;

use bot_any_cal::Command;
use bot_any_platform_discord::sys::{
    commands::{DeleteCommand, ListCommands, UpdateCommand},
    types::{ApplicationCommand, Snowflake},
};
use crowdin_strife::commands::{RootCommand, TestCommand, Version, WorksLeft};
use reqores_client_surf::SurfClient;
use tokio::time;

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
                guild_id: command.guild_id,
            })
            .await
            .map_err(|e| eyre::eyre!("{}", e))?;
    }

    println!();

    let commands = vec![
        ApplicationCommand::try_from(WorksLeft::spec())?,
        ApplicationCommand::try_from(Version::spec())?,
        ApplicationCommand::try_from(TestCommand::spec())?,
    ];

    for command in commands {
        let result = client
            .call(UpdateCommand {
                application_id: &discord_application_id,
                token: &discord_token,
                // guild_id: None,
                guild_id: guild_id.clone(),
                command,
            })
            .await
            .map_err(|e| eyre::eyre!("{}", e))?;
        println!("Successfully registered command '{}'", result.name);
        time::sleep(Duration::from_millis(100)).await;
    }

    Ok(())
}
