use bot_any_platform_discord::{
    sys::{
        types::{Interaction, InteractionCallbackMessage, InteractionResponse, RawInteraction},
        verify_key::VerifyKey,
    },
    DiscordGarden,
};
use crowdin_client::{DiscussionStatus, LanguageId, LoadTopics, RefreshToken};
use reqores::{ServerRequest, ServerResponse, ServerResponseBuilder, StatusCode};
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
            let discord_garden = DiscordGarden::new(context.var("DISCORD_PUBLIC_KEY")?)?;

            let request = CfWorkerServerRequest::new(req).await?;
            make_response(discord_garden.accept(&request).await?)
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

async fn discord(request: &impl ServerRequest) -> worker::Result<ServerResponse> {
    let interaction: RawInteraction = server_request.body_json()?;
    match interaction.transform() {
        Some(Interaction::Ping) => Ok(ServerResponseBuilder::new()
            .status(StatusCode::Ok)
            .body_json(&InteractionResponse::pong())?),
        Some(Interaction::ApplicationCommand(_)) => Ok(ServerResponseBuilder::new()
            .status(StatusCode::Ok)
            .body_json(&InteractionResponse::message_with_source(works_left()))?),
        None => Ok(ServerResponseBuilder::new()
            .status(StatusCode::BadRequest)
            .body("wtf".as_bytes().to_vec())),
    }
}

async fn works_left() -> InteractionCallbackMessage {
    InteractionCallbackMessage {
        tts: None,
        content: Some("Hello, world!".to_string()),
        embeds: vec![],
    }
}
