use bot_any_platform_discord::sys::types::{
    Interaction, InteractionCallbackMessage, InteractionResponse, RawInteraction,
};
use crowdin_client::{DiscussionStatus, LanguageId, LoadTopics, RefreshToken};
use reqores::ServerRequest;
use reqores_client_cf_worker::CfWorkerClient;
use reqores_server_cf_worker::CfWorkerServerRequest;
use worker::{event, Env, Request, Response, Result, Router};

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    let router = Router::new();
    router
        .post_async("/discord/interactions", |req, _| async move {
            let server_request = CfWorkerServerRequest::new(req).await?;

            let interaction: RawInteraction = server_request.body_json()?;
            match interaction.process() {
                Some(Interaction::Ping) => Response::from_json(&InteractionResponse::pong()),
                Some(Interaction::ApplicationCommand(_)) => Response::from_json(
                    &InteractionResponse::message_with_source(InteractionCallbackMessage {
                        tts: None,
                        content: Some("Hello, world!".to_string()),
                        embeds: vec![],
                    }),
                ),
                None => Response::from_html("wtf"),
            }
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
