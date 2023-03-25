use engine::db::{MinecraftPlatform, SearchTmQuery, SourceLanguage, TmDatabase};

use crate::message::StructuredMessageBox;

pub async fn search_tm(
    api: &(impl TmDatabase + Sync + Send),
    platform: MinecraftPlatform,
    source: SourceLanguage,
    query: String,
    page: Option<i64>,
) -> eyre::Result<StructuredMessageBox> {
    let query = query.to_lowercase();
    let page = page.unwrap_or(1) - 1;

    let res = api
        .search(SearchTmQuery {
            source,
            platform,
            text: query.clone(),
            skip: 10 * page as usize,
            take: 10,
        })
        .await?;
    let total_pages = (res.list.total + 9) / 10;

    let mut message = String::new();
    message.push_str(&format!("{} ▷ {}\n", res.game_version, query));

    for entry in &res.list.items {
        message.push_str(&format!(
            "{} => {}\n",
            entry.source.content,
            entry
                .targets
                .get(0)
                .map(|target| &*target.content)
                .unwrap_or("*번역 없음*")
        ));
    }

    if res.list.items.is_empty() {
        message.push_str(&"결과 없음".to_string());
    } else if total_pages > 1 {
        message.push_str(&format!("페이지 {} / {}", page + 1, total_pages));
    }

    Ok(Box::new(message))
}
