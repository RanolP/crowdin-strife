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
) -> Vec<String> {
    vec![
        platform.name().to_string(),
        source.id().to_string(),
        target.id().to_string(),
        query,
        page.to_string(),
    ]
}

pub fn try_deserialize(
    values: Vec<String>,
) -> Option<(MinecraftPlatform, Language, Language, String, i64)> {
    let platform = MinecraftPlatform::from_name(&values.get(0)?)?;
    let source = Language::from_id(&values.get(1)?)?;
    let target = Language::from_id(&values.get(2)?)?;
    let query = values.get(3)?.clone();
    let page = values.get(4)?.parse().ok()?;

    Some((platform, source, target, query, page))
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
            platform,
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

        entries: res.list.items,

        page: page as usize,
        total_pages,
    }))
}
