use bot_any::types::MessageWrite;
use bot_any_env_cf_worker::CfWorkerEnv;
use bot_any_platform_discord::{
    kal::parse_command, sys::types::InteractionResponse, DiscordFruit, DiscordGarden,
};
use crowdin_client::{
    CrowdinResponse, DiscussionStatus, LanguageId, LoadTopics, LoadTopicsResponse, RefreshToken,
};
use kal::Command;
use reqores::{HttpStatusCode, ServerResponseBuilder};
use reqores_universal_cf_worker::{client::CfWorkerClient, server};
use worker::{Context, Env, Request, Response, Router};

use crate::commands::{handle_unknown, RootCommand};

pub async fn actual_main(req: Request, env: Env, _ctx: Context) -> worker::Result<Response> {
    Router::new()
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

            let response = match garden
                .plant(&server::decode_request(req).await?)
                .await
                .map_err(|e| worker::Error::from(e.to_string()))?
            {
                (res, DiscordFruit::EarlyReturn) => res,
                (res, DiscordFruit::Command(command)) => {
                    let (sender, preflights) = parse_command(command);
                    let env = CfWorkerEnv(context.env);
                    let message_output = if let Some(command) = RootCommand::parse(&preflights) {
                        match command.execute(sender, &env).await {
                            Ok(output) => output,
                            Err(err) => MessageWrite::begin()
                                .push_str(format!(
                                    "명령어 실행에 실패했습니다:\n```\n{}\n```",
                                    err.to_string()
                                ))
                                .end(),
                        }
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
            Response::from_json(&list_discussions().await?)
        })
        .run(req, env)
        .await
}

async fn list_discussions() -> worker::Result<CrowdinResponse<LoadTopicsResponse>> {
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
        .await?;

    Ok(response)
}
