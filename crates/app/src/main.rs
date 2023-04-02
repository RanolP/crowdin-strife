use std::env;

use app::commands::{handle_unknown, RootCommand};
use engine::{
    db::{PrismaDatabase, TmDatabase},
    env::{Env, StdEnv},
};
use kal::Command;
use kal_serenity::{parse_command, try_into_serenity_command};
use serenity::{
    async_trait,
    model::application::interaction::InteractionResponseType,
    model::prelude::{command::Command as SerenityCommand, interaction::Interaction, Ready},
    prelude::*,
};

struct Handler<E, Db> {
    env: E,
    database: Db,
}

#[async_trait]
impl<E, Db> EventHandler for Handler<E, Db>
where
    E: Env + Sync + Send,
    Db: TmDatabase + Sync + Send,
{
    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        match interaction {
            Interaction::Ping(_) => {}
            Interaction::ApplicationCommand(interaction) => {
                let preflights = parse_command(&interaction.data);

                let message_output = if let Ok(command) = RootCommand::parse(&preflights) {
                    match command.execute(&self.env, &self.database).await {
                        Ok(output) => output,
                        Err(err) => Box::new(format!(
                            "명령어 실행에 실패했습니다:\n```\n{}\n```",
                            err.to_string()
                        )),
                    }
                } else {
                    handle_unknown(&preflights).await
                };

                interaction
                    .create_interaction_response(&ctx.http, |response| {
                        response
                            .kind(InteractionResponseType::ChannelMessageWithSource)
                            .interaction_response_data(move |data| {
                                message_output.write_boxed_into(data)
                            })
                    })
                    .await
                    .unwrap();
            }
            Interaction::MessageComponent(message_component) => {
                dbg!(
                    ctx.http
                        .get_message(
                            message_component.channel_id.0,
                            message_component.message.id.0
                        )
                        .await
                        .unwrap()
                        .interaction
                );
                match &*message_component.data.custom_id {
                    "prev" => {}
                    "next" => {}
                    _ => {
                        println!("wtf");
                    }
                }
            }
            Interaction::Autocomplete(_) => {}
            Interaction::ModalSubmit(_) => {}
        }
    }

    async fn ready(&self, ctx: Context, _ready: Ready) {
        SerenityCommand::set_global_application_commands(&ctx.http, |commands| {
            for command in RootCommand::children_specs() {
                commands.add_application_command(try_into_serenity_command(command).unwrap());
            }
            commands
        })
        .await
        .unwrap();
    }
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    let is_production = env::var("ENVIRONMENT")
        .map(|x| x == "production")
        .unwrap_or(false);
    let public_key = is_production
        .then(|| env::var("DISCORD_PUBLIC_KEY"))
        .transpose()?;
    let token = env::var("DISCORD_TOKEN")?;
    let application_id: u64 = env::var("DISCORD_APP_ID")?.parse()?;

    let database = PrismaDatabase::connect().await?;

    let env = StdEnv;
    let mut client = Client::builder(token, GatewayIntents::empty())
        .application_id(application_id)
        .event_handler(Handler { env, database })
        .await?;

    println!(
        "Invite bot with https://discord.com/api/oauth2/authorize?client_id={}&permissions={}&scope={}",
        application_id, 0, "bot%20applications.commands"
    );

    client.start().await?;

    Ok(())
}
