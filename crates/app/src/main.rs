use std::env;

use app::{
    commands::{handle_unknown, RootCommand},
    e2k_base::{search_tm, try_deserialize},
    message::Render,
    msgdata::decode_msgdata,
};
use engine::{
    db::{PrismaDatabase, TmDatabase},
    env::{Env, LayeredEnv, PredefinedEnv, StdEnv},
};
use kal::Command;
use kal_serenity::{parse_command, try_into_serenity_command};
use serenity::{
    async_trait,
    model::application::interaction::InteractionResponseType,
    model::{
        prelude::{command::Command as SerenityCommand, interaction::Interaction, Ready},
        Permissions,
    },
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
                            .interaction_response_data(move |data| data.render(&*message_output))
                    })
                    .await
                    .unwrap();
            }
            Interaction::MessageComponent(message_component) => {
                let fetch_msgdata = || {
                    message_component
                        .message
                        .embeds
                        .get(0)
                        .and_then(|embed| embed.description.as_ref())
                        .and_then(|raw| decode_msgdata(raw))
                        .and_then(|msgdata| try_deserialize(&msgdata))
                };
                match &*message_component.data.custom_id {
                    "prev" => {
                        let Some((platform, source, target, query, page, total_pages)) = fetch_msgdata() else {
                             return
                        };
                        let res = search_tm(
                            &self.database,
                            platform,
                            source,
                            target,
                            query,
                            if page > 0 { Some(page - 1) } else { Some(0) },
                        )
                        .await
                        .unwrap();
                        message_component
                            .create_interaction_response(ctx.http, |response| {
                                response.interaction_response_data(|data| data.render(&*res))
                            })
                            .await
                            .unwrap();
                    }
                    "next" => {
                        let Some((platform, source, target, query, page, total_pages)) = fetch_msgdata() else {
                             return
                        };
                        let res = search_tm(
                            &self.database,
                            platform,
                            source,
                            target,
                            query,
                            if page < total_pages {
                                Some(page + 1)
                            } else {
                                Some(0)
                            },
                        )
                        .await
                        .unwrap();
                        message_component
                            .create_interaction_response(ctx.http, |response| {
                                response.interaction_response_data(|data| data.render(&*res))
                            })
                            .await
                            .unwrap();
                    }
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

    let version = env!("CARGO_PKG_VERSION");

    let database = PrismaDatabase::connect().await?;

    let env = LayeredEnv((
        PredefinedEnv::new().with("VERSION".to_string(), version.to_string()),
        StdEnv,
    ));
    let mut client = Client::builder(token, GatewayIntents::empty())
        .application_id(application_id)
        .event_handler(Handler { env, database })
        .await?;

    println!(
        "Invite bot with https://discord.com/api/oauth2/authorize?client_id={}&permissions={}&intents={}&scope={}",
        application_id, Permissions::default().bits(), GatewayIntents::default().bits(), "bot%20applications.commands",
    );

    client.start().await?;

    Ok(())
}
