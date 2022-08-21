use std::borrow::Cow;

use reqores::{ClientRequest, HttpMethod, Url};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

use crate::{CrowdinResponse, DiscussionId, DiscussionStatus, LanguageId, UserId, BASE_URL};

pub struct LoadTopics<'a> {
    pub csrf_token: &'a str,

    pub project_id: u32,
    pub status: Option<DiscussionStatus>,

    pub language_id: Option<LanguageId>,
    pub author_id: Option<UserId>,
}

#[derive(Serialize, Deserialize)]
pub struct LoadTopicsResponse {
    topics: Vec<Topic>,
    pager: Pager,
}

#[serde_as]
#[derive(Serialize, Deserialize)]
pub struct Topic {
    id: DiscussionId,
    name: String,
    language_id: LanguageId,

    // TODO: Date Parse?
    created_at: String,
    updated_at: String,

    #[serde_as(as = "DisplayFromStr")]
    replies_count: u32,
}

#[derive(Serialize, Deserialize)]
pub struct Pager {
    total: u32,
    current_page: u32,
    per_page: u32,
}

impl ClientRequest for LoadTopics<'_> {
    type Response = CrowdinResponse<LoadTopicsResponse>;

    fn headers(&self) -> Vec<(String, String)> {
        vec![
            // TODO: X-Csrf-Token ok? I think it's just random value so we could lock
            ("X-Csrf-Token".to_string(), "nqys8q8d2x".to_string()),
            (
                "Cookie".to_string(),
                format!("CSRF-TOKEN={}; csrf_token=nqys8q8d2x", self.csrf_token),
            ),
        ]
    }

    fn url(&self) -> Cow<str> {
        let mut url = Url::parse(&format!("{}/discussions/load_topics", BASE_URL)).unwrap();

        url.query_pairs_mut()
            .append_pair("project_id", &self.project_id.to_string());

        if let Some(status) = &self.status {
            url.query_pairs_mut().append_pair("status", status.as_str());
        }

        if let Some(language_id) = self.language_id.clone() {
            url.query_pairs_mut()
                .append_pair("language_id", &language_id.0.to_string());
        }
        if let Some(author_id) = &self.author_id {
            url.query_pairs_mut().append_pair("status", &author_id.0);
        }

        Cow::Owned(url.to_string())
    }

    fn method(&self) -> &HttpMethod {
        &HttpMethod::Get
    }
}
