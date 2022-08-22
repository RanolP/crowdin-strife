use bot_any_platform_discord::sys::{
    types::{Interaction, InteractionCallbackMessage, InteractionResponse, RawInteraction},
    verify_key::VerifyKey,
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
        .post_async("/discord/interactions", |req, _| async move {
            let verify_key = VerifyKey::new(env.var("DISCORD_PUBLIC_KEY")?)?;

            let server_request = CfWorkerServerRequest::new(req).await?;
            let server_response = verify_key
                .accept(&server_request)
                .await?
                .then(discord(&server_request).await?);
            let response = make_response(server_response)?;

            response
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
            .body_json(&InteractionResponse::message_with_source(
                InteractionCallbackMessage {
                    tts: None,
                    content: Some("Hello, world!".to_string()),
                    embeds: vec![],
                },
            ))?),
        None => Ok(ServerResponseBuilder::new()
            .status(StatusCode::BadRequest)
            .body("wtf".as_bytes().to_vec())),
    }
}
