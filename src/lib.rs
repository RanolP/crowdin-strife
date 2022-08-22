use bot_any::types::MessageOutput;
use bot_any_platform_discord::{
    sys::types::{ApplicationCommand, InteractionResponse},
    DiscordGarden, DiscordPlant,
};
use crowdin_client::{DiscussionStatus, LanguageId, LoadTopics, RefreshToken};
use reqores::{ServerResponse, ServerResponseBuilder, StatusCode};
use reqores_client_cf_worker::CfWorkerClient;
use reqores_server_cf_worker::{make_response, CfWorkerServerRequest};
use worker::{event, Env, Request, Response, Result, Router};

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    let router = Router::new();
    router
        .post_async("/discord/interactions", |req, context| async move {
            let garden = DiscordGarden::new(&context.var("DISCORD_PUBLIC_KEY")?.to_string())
                .map_err(|e| worker::Error::from(e.to_string()))?;

            let request = CfWorkerServerRequest::new(req).await?;
            let response = match garden
                .accept(&request)
                .await
                .map_err(|e| worker::Error::from(e.to_string()))?
            {
                (res, DiscordPlant::EarlyReturn) => res,
                (res, DiscordPlant::Command(command)) => res.then(execute(command).await?),
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

async fn execute(command: ApplicationCommand) -> worker::Result<ServerResponse> {
    Ok(ServerResponseBuilder::new()
        .status(StatusCode::Ok)
        .body_json(&InteractionResponse::message_with_source(
            works_left().await.into(),
        ))?)
}

async fn works_left() -> MessageOutput {
    MessageOutput {
        content: Some("Hello, world!".to_string()),
    }
}
