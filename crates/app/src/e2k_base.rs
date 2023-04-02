use engine::{
    db::{MinecraftPlatform, SearchTmQuery, TmDatabase},
    language::Language,
};

use crate::{lookup::message::LookupResult, message::BoxedStructuredMessage};

pub fn serialize(
    platform: MinecraftPlatform,
    source: Language,
    target: Language,
    query: String,
    page: i64,
    total_pages: i64,
) -> String {
    serde_json::to_string(&(
        platform.id().to_string(),
        source.id().to_string(),
        target.id().to_string(),
        query,
        page,
        total_pages,
    ))
    .unwrap()
}

pub fn try_deserialize(
    s: &str,
) -> Option<(MinecraftPlatform, Language, Language, String, i64, i64)> {
    let (platform, source, target, query, page, total_pages): (
        String,
        String,
        String,
        String,
        i64,
        i64,
    ) = serde_json::from_str(s).ok()?;
    let platform = MinecraftPlatform::from_id(&platform)?;
    let source = Language::from_id(&source)?;
    let target = Language::from_id(&target)?;

    Some((platform, source, target, query, page, total_pages))
}

pub async fn search_tm(
    api: &(impl TmDatabase + Sync + Send),
    platform: MinecraftPlatform,
    source: Language,
    target: Language,
    query: String,
    page: Option<i64>,
) -> eyre::Result<BoxedStructuredMessage> {
    let query = query.to_lowercase();
    let page = page.unwrap_or(1);

    let res = api
        .search(SearchTmQuery {
            source: source.clone(),
            target: target.clone(),
            platform: platform.clone(),
            text: query.clone(),
            skip: 10 * (page - 1) as usize,
            take: 10,
        })
        .await?;
    let total_pages = (res.list.total + 9) / 10;

    Ok(Box::new(LookupResult {
        query,

        source_language: source,
        target_language: target,

        game_version: res.game_version,
        platform,

        entries: res.list.items,

        page: page as usize,
        total_pages,
    }))
}
