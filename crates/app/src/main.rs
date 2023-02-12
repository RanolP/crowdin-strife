use commands::{handle_unknown, RootCommand};
use kal_serenity::parse_command;
use serenity::{
    async_trait, model::application::interaction::InteractionResponseType,
    model::prelude::interaction::Interaction, prelude::*, Client,
};

pub mod commands;
pub mod e2k_base;
pub mod file_reader;

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::Ping(_) => {}
            Interaction::ApplicationCommand(interaction) => {
                let preflights = parse_command(&interaction.data);
                let env = CfWorkerEnv(&context.env);
                let message_output = if let Ok(command) = RootCommand::parse(&preflights) {
                    let asset_store = AssetStore(&context.env);
                    match command.execute(&env, &asset_store).await {
                        Ok(output) => output,
                        Err(err) => {
                            format!("명령어 실행에 실패했습니다:\n```\n{}\n```", err.to_string())
                        }
                    }
                } else {
                    handle_unknown(&preflights).await
                };

                interaction.create_interaction_response(&ctx.http, |response| {
                    response
                        .kind(InteractionResponseType::ChannelMessageWithSource)
                        .interaction_response_data(|data| data.content(message_output))
                })
            }
            Interaction::MessageComponent(_) => {}
            Interaction::Autocomplete(_) => {}
            Interaction::ModalSubmit(_) => {}
        }
    }
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenvy::dotenv();
    let is_production = dotenvy::var("ENVIRONMENT")? != "development";
    let public_key = is_production
        .then(|| dotenvy::var("DISCORD_PUBLIC_KEY"))
        .transpose()?;
    let token = dotenvy::var("DISCORD_TOKEN")?;
    let application_id: u64 = dotenvy::var("DISCORD_APP_ID")?.parse()?;

    let client = Client::builder(token, GatewayIntents::empty())
        .application_id(application_id)
        .event_handler(Handler)
        .await?;

    client.start().await?;

    Ok(())
}
