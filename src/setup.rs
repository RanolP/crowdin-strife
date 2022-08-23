#![cfg(not(target_arch = "wasm32"))]

use bot_any_platform_discord::sys::{
    commands::{DeleteCommand, UpdateCommand},
    types::{
        ApplicationCommand, ApplicationCommandKind, ApplicationCommandOption,
        ApplicationCommandOptionKind, Snowflake,
    },
};
use reqores_client_surf::SurfClient;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenvy::dotenv().ok();
    color_eyre::install();

    let discord_application_id = dotenvy::var("DISCORD_APPLICATION_ID")?;
    let discord_token = dotenvy::var("DISCORD_TOKEN")?;

    let client = SurfClient;

    client
        .call(DeleteCommand {
            application_id: &discord_application_id,
            token: &discord_token,
            command_id: Snowflake("1010918050310135960".to_string()),
        })
        .await
        .map_err(|e| eyre::eyre!("{}", e))?;

    let result = client
        .call(UpdateCommand {
            application_id: &discord_application_id,
            token: &discord_token,
            // guild_id: None,
            guild_id: Some(Snowflake("898200418146988072".to_string())),
            command: ApplicationCommand {
                id: None,
                kind: Some(ApplicationCommandKind::ChatInput),
                application_id: Some(Snowflake(discord_application_id.clone())),
                guild_id: None,
                name: "잔업".to_string(),
                options: vec![],
                description: Some("아직 기능이 없어요 ㅠ".to_string()),
            },
        })
        .await
        .map_err(|e| eyre::eyre!("{}", e))?;
    println!("{}", serde_json::to_string_pretty(&result)?);

    let result = client
        .call(UpdateCommand {
            application_id: &discord_application_id,
            token: &discord_token,
            // guild_id: None,
            guild_id: Some(Snowflake("898200418146988072".to_string())),
            command: ApplicationCommand {
                id: None,
                kind: Some(ApplicationCommandKind::ChatInput),
                application_id: Some(Snowflake(discord_application_id.clone())),
                guild_id: None,
                name: "버전".to_string(),
                options: vec![],
                description: Some("버전 정보를 가져옵니다.".to_string()),
            },
        })
        .await
        .map_err(|e| eyre::eyre!("{}", e))?;
    println!("{}", serde_json::to_string_pretty(&result)?);

    let result = client
        .call(UpdateCommand {
            application_id: &discord_application_id,
            token: &discord_token,
            // guild_id: None,
            guild_id: Some(Snowflake("898200418146988072".to_string())),
            command: ApplicationCommand {
                id: None,
                kind: Some(ApplicationCommandKind::ChatInput),
                application_id: Some(Snowflake(discord_application_id.clone())),
                guild_id: None,
                name: "cmdframework".to_string(),
                options: vec![
                    ApplicationCommandOption {
                        kind: ApplicationCommandOptionKind::SubCommand,
                        name: "sc1".to_string(),
                        description: Some("서브커맨드1".to_string()),
                        required: Some(false),
                        choices: vec![],
                        options: vec![],
                    },
                    ApplicationCommandOption {
                        kind: ApplicationCommandOptionKind::SubCommand,
                        name: "sc2".to_string(),
                        description: Some("서브커맨드2".to_string()),
                        required: Some(false),
                        choices: vec![],
                        options: vec![ApplicationCommandOption {
                            kind: ApplicationCommandOptionKind::Integer,
                            name: "test".to_string(),
                            description: Some("테스트값".to_string()),
                            required: Some(false),
                            choices: vec![],
                            options: vec![],
                        }],
                    },
                ],
                description: Some("버전 정보를 가져옵니다.".to_string()),
            },
        })
        .await
        .map_err(|e| eyre::eyre!("{}", e))?;
    println!("{}", serde_json::to_string_pretty(&result)?);

    Ok(())
}
