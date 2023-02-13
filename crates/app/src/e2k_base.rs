use engine::api::{CrowdinStrifeApi, MinecraftPlatform, SearchTmQuery, SourceLanguage};

pub async fn search_tm(
    api: &dyn CrowdinStrifeApi,
    platform: MinecraftPlatform,
    source: SourceLanguage,
    query: String,
    page: Option<i64>,
) -> eyre::Result<String> {
    let query = query.to_lowercase();
    let page = page.unwrap_or(1) - 1;

    let res = api
        .search_tm(SearchTmQuery {
            source,
            platform,
            text: query.clone(),
            skip: 10 * page as usize,
            take: 10,
        })
        .await;
    let total_pages = (res.total + 9) / 10;

    let mut message = String::new();
    message.push_str(&format!("▷ {}\n", query));

    for entry in &res.items {
        message.push_str(&format!(
            "{} => {}\n",
            entry.source.content, entry.targets[0].content
        ));
    }

    if res.items.is_empty() {
        message.push_str(&"결과 없음".to_string());
    } else if total_pages > 1 {
        message.push_str(&format!("페이지 {} / {}", page + 1, total_pages));
    }

    Ok(message)
}
