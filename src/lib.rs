pub mod commands;

#[cfg(target_arch = "wasm32")]
#[worker::event(fetch)]
pub async fn main(
    req: worker::Request,
    env: worker::Env,
    _ctx: worker::Context,
) -> worker::Result<worker::Response> {
    use crate::commands::{handle_unknown, RootCommand};
    use bot_any_cal::Command;
    use bot_any_env_cf_worker::CfWorkerEnv;
    use bot_any_platform_discord::{
        cal::parse_command, sys::types::InteractionResponse, DiscordGarden, DiscordPlant,
    };
    use crowdin_client::{DiscussionStatus, LanguageId, LoadTopics, RefreshToken};
    use reqores::{ServerResponseBuilder, StatusCode};
    use reqores_client_cf_worker::CfWorkerClient;
    use reqores_server_cf_worker::{make_response, CfWorkerServerRequest};
    use worker::{Response, Router};

    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    let router = Router::new();
    router
        .post_async("/discord/interactions", |req, context| async move {
            let debug = context.var("ENVIRONMENT")?.to_string() == "development";
            let public_key = if debug {
                None
            } else {
                Some(context.secret("DISCORD_PUBLIC_KEY")?.to_string())
            };
            let garden = DiscordGarden::new(public_key.as_deref())
                .map_err(|e| worker::Error::from(e.to_string()))?;

            let request = CfWorkerServerRequest::new(req).await?;
            let response = match garden
                .seed(&request)
                .await
                .map_err(|e| worker::Error::from(e.to_string()))?
            {
                (res, DiscordPlant::EarlyReturn) => res,
                (res, DiscordPlant::Command(command)) => {
                    let (sender, preflights) = parse_command(command);
                    let env = CfWorkerEnv(context.env);
                    let message_output = if let Some(command) = RootCommand::parse(&preflights) {
                        match command {
                            RootCommand::TestCommand(command) => {
                                command.execute(sender, &env).await
                            }
                            RootCommand::WorksLeft(works_left) => {
                                works_left.execute(sender, &env).await
                            }
                            RootCommand::Version(version) => version.execute(sender, &env).await,
                        }
                    } else {
                        handle_unknown(sender, &preflights, &env).await
                    };

                    res.then(
                        ServerResponseBuilder::new()
                            .status(StatusCode::Ok)
                            .body_json(&InteractionResponse::message_with_source(
                                message_output.into(),
                            ))?,
                    )
                }
            };

            make_response(response)
        })
        .get_async("/discussions/list", |_, _| async {
            let client = CfWorkerClient;

            let csrf_token = client.call(RefreshToken).await?;

            let response = client
                .call(LoadTopics {
                    csrf_token: &csrf_token,
                    project_id: 3579,
                    status: Some(DiscussionStatus::Open),
                    language_id: Some(LanguageId(27)),
                    author_id: None,
                })
                .await
                .unwrap();

            Response::from_json(&response)
        })
        .run(req, env)
        .await
}
