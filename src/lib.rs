pub mod commands;

#[cfg(target_arch = "wasm32")]
extern crate wee_alloc;

#[cfg(target_arch = "wasm32")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[cfg(target_arch = "wasm32")]
#[worker::event(fetch)]
pub async fn main(
    req: worker::Request,
    env: worker::Env,
    _ctx: worker::Context,
) -> worker::Result<worker::Response> {
    use crate::commands::{handle_unknown, RootCommand};
    use bot_any_env_cf_worker::CfWorkerEnv;
    use bot_any_platform_discord::{
        kal::parse_command, sys::types::InteractionResponse, DiscordFruit, DiscordGarden,
    };
    use crowdin_client::{DiscussionStatus, LanguageId, LoadTopics, RefreshToken};
    use kal::Command;
    use reqores::{HttpStatusCode, ServerResponseBuilder};
    use reqores_universal_cf_worker::{client::CfWorkerClient, server};
    use worker::{Response, Router};

    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    let router = Router::new();
    router
        .post_async("/discord/interactions", |req, context| async move {
            let is_production = context.var("ENVIRONMENT")?.to_string() != "development";
            let public_key = is_production
                .then(|| {
                    context
                        .secret("DISCORD_PUBLIC_KEY")
                        .map(|binding| binding.to_string())
                })
                .transpose()?;
            let garden = DiscordGarden::new(public_key.as_deref())
                .map_err(|e| worker::Error::from(e.to_string()))?;

            let request = server::decode_request(req).await?;
            let response = match garden
                .plant(&request)
                .await
                .map_err(|e| worker::Error::from(e.to_string()))?
            {
                (res, DiscordFruit::EarlyReturn) => res,
                (res, DiscordFruit::Command(command)) => {
                    let (sender, preflights) = parse_command(command);
                    let env = CfWorkerEnv(context.env);
                    let message_output = if let Some(command) = RootCommand::parse(&preflights) {
                        command.execute(sender, &env).await
                    } else {
                        handle_unknown(sender, &preflights, &env).await
                    };

                    res.then(
                        ServerResponseBuilder::new()
                            .with_status(HttpStatusCode::Ok)
                            .body_json(&InteractionResponse::message_with_source(
                                message_output.into(),
                            ))?,
                    )
                }
            };

            server::encode_response(response)
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
