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

pub struct SearchTmQuery {
    pub source: SourceLanguage,
    pub text: String,
}
pub struct SearchTmResultEntry {}

#[async_trait]
pub trait CrowdinStrifeApi {
    async fn search_tm(&self, query: SearchTmQuery) -> Vec<SearchTmResultEntry>;
}
