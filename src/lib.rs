use crowdin_client::{DiscussionStatus, LanguageId, LoadTopics, RefreshToken};
use reqores_client_cf_worker::CfWorkerClient;
use worker::{event, Env, Headers, Request, Response, Result};

#[event(fetch)]
pub async fn main(req: Request, env: Env, _ctx: worker::Context) -> Result<Response> {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();

    let client = CfWorkerClient;

    let csrf_token = client.get(RefreshToken).await?;

    let topics = LoadTopics {
        csrf_token: &csrf_token,
        project_id: 3579,
        status: Some(DiscussionStatus::Open),
        language_id: Some(LanguageId::KOREAN),
        author_id: None,
    };

    let response = client.get(topics).await.unwrap();

    let mut headers = Headers::new();

    headers.set("Content-Type", "application/json; charset=UTF-8")?;

    Ok(Response::from_html(serde_json::to_string(&response).unwrap())?.with_headers(headers))
}
