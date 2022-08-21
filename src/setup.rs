use bot_any_platform_discord::sys::{
    commands::{DeleteCommand, UpdateCommand},
    types::{ApplicationCommand, ApplicationCommandKind, Snowflake},
};
use reqores_client_surf::SurfClient;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let discord_application_id = dotenvy::var("DISCORD_APPLICATION_ID").unwrap();
    let discord_token = dotenvy::var("DISCORD_TOKEN").unwrap();

    let client = SurfClient;

    client
        .call(DeleteCommand {
            application_id: &discord_application_id,
            token: &discord_token,
            command_id: Snowflake("1010918050310135960".to_string()),
        })
        .await
        .unwrap();

    let result = client
        .call(UpdateCommand {
            application_id: &discord_application_id,
            token: &discord_token,
            // guild_id: None,
            guild_id: Some(Snowflake("898200418146988072".to_string())),
            command: ApplicationCommand {
                id: None,
                kind: Some(ApplicationCommandKind::ChatInput),
                application_id: Snowflake(discord_application_id.clone()),
                guild_id: None,
                name: "잔업".to_string(),
                options: None,
                description: "아직 기능이 없어요 ㅠ".to_string(),
            },
        })
        .await
        .unwrap();

    println!("{}", serde_json::to_string_pretty(&result).unwrap());
}
