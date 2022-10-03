#![cfg(not(target_arch = "wasm32"))]

use std::time::Duration;

use bot_any_platform_discord::sys::{
    commands::{DeleteCommand, ListCommands, UpdateCommand},
    error::{DiscordError, DiscordResult},
    types::{ApplicationCommand, Snowflake},
};
use crowdin_strife::commands::RootCommand;
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

    let client = SurfClient::new();

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

    for command in RootCommand::children_specs() {
        let update_command = UpdateCommand {
            application_id: &discord_application_id,
            token: &discord_token,
            // guild_id: None,
            guild_id: guild_id.clone(),
            command: ApplicationCommand::try_from(command)?,
        };
        loop {
            let result = client
                .call(update_command.clone())
                .await
                .map_err(|e| eyre::eyre!("{}", e))?;
            match result {
                DiscordResult::Ok(result) => {
                    println!("Successfully registered command '{}'", result.name);
                    break;
                }
                DiscordResult::Err(e) => match e {
                    DiscordError::Coded { code, message } => {
                        eyre::bail!(
                            "Failed to register commanbd with code {}: {}",
                            code,
                            message
                        );
                    }
                    DiscordError::RateLimited { retry_after, .. } => {
                        println!("Rated limited: sleep {}s", retry_after);
                        time::sleep(Duration::from_secs(1).mul_f64(retry_after)).await;
                    }
                    DiscordError::Unknown(e) => {
                        eyre::bail!("Failed to register commanbd: {}", e);
                    }
                },
            }
        }
    }

    Ok(())
}
