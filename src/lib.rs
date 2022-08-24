pub mod commands;

#[cfg(target_arch = "wasm32")]
#[worker::event(fetch)]
pub async fn main(
    req: worker::Request,
    env: worker::Env,
    _ctx: worker::Context,
) -> worker::Result<worker::Response> {
    use bot_any::types::MessageOutput;
    use bot_any_platform_discord::{sys::types::InteractionResponse, DiscordGarden, DiscordPlant};
    use crowdin_client::{DiscussionStatus, LanguageId, LoadTopics, RefreshToken};
    use reqores::{ServerResponse, ServerResponseBuilder, StatusCode};
    use reqores_client_cf_worker::CfWorkerClient;
    use reqores_server_cf_worker::{make_response, CfWorkerServerRequest};
    use worker::{Response, RouteContext, Router};

    async fn execute(context: RouteContext<()>) -> worker::Result<ServerResponse> {
        Ok(ServerResponseBuilder::new()
            .status(StatusCode::Ok)
            .body_json(&InteractionResponse::message_with_source(
                match command.label.as_ref() {
                    "잔업" => works_left(command, context).await,
                    "버전" => version(command, context).await,
                    _ => unknown(command, context).await,
                }?
                .into(),
            ))?)
    }

    async fn works_left(context: RouteContext<()>) -> worker::Result<MessageOutput> {
        Ok(MessageOutput {
            content: Some("잔업은 언젠가 완료될 것입니다.".to_string()),
        })
    }

    async fn version(context: RouteContext<()>) -> worker::Result<MessageOutput> {
        Ok(MessageOutput {
            content: Some(format!("버전 : {}", context.var("VERSION")?.to_string())),
        })
    }

    async fn unknown(context: RouteContext<()>) -> worker::Result<MessageOutput> {
        Ok(MessageOutput {
            content: Some("알 수 없는 명령어입니다.".to_string()),
        })
    }

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
                .accept(&request)
                .await
                .map_err(|e| worker::Error::from(e.to_string()))?
            {
                (res, DiscordPlant::EarlyReturn) => res,
                (res, DiscordPlant::Command(command)) => {
                    res.then(execute(command.into(), context).await?)
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
