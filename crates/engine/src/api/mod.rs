use async_trait::async_trait;

pub enum Language {
    English,
    Korean,
}

pub enum SourceLanguage {
    Specified(Language),
    Auto,
}

impl Default for SourceLanguage {
    fn default() -> Self {
        SourceLanguage::Auto
    }
}

pub enum MinecraftPlatform {
    Java,
    Bedrock,
    Dungeons,
}

pub struct SearchTmQuery {
    pub source: SourceLanguage,
    pub platform: MinecraftPlatform,
    pub text: String,
    pub skip: usize,
    pub take: usize,
}

pub struct Pagination<T> {
    pub total: usize,
    pub items: Vec<T>,
}
pub struct SearchTmResultEntry {
    pub key: String,
    pub source: TmEntry,
    pub targets: Vec<TmEntry>,
}

pub struct TmEntry {
    pub language: Language,
    pub content: String,
}

#[async_trait]
pub trait CrowdinStrifeApi {
    async fn search_tm(&self, query: SearchTmQuery) -> Pagination<SearchTmResultEntry>;
}
