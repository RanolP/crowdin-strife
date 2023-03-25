use async_trait::async_trait;
use once_cell::sync::Lazy;
pub use r#impl::*;
use regex::Regex;

mod r#impl;

#[derive(Debug, Clone)]
pub enum Language {
    English,
    Korean,
}

impl Language {
    pub fn as_e2k_counterpart(&self) -> Language {
        match self {
            Language::English => Language::Korean,
            Language::Korean => Language::English,
        }
    }
    pub fn as_str(&self) -> &str {
        match self {
            Language::English => "en-US",
            Language::Korean => "ko-KR",
        }
    }
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

static HANGUL: Lazy<Regex> = Lazy::new(|| Regex::new(r#"[가-힣ㄱ-ㅎㅏ-ㅣ]"#).unwrap());

impl SourceLanguage {
    pub fn guess(self, text: &str) -> Language {
        match self {
            SourceLanguage::Specified(language) => language,
            SourceLanguage::Auto => {
                if HANGUL.is_match(text) {
                    Language::Korean
                } else {
                    Language::English
                }
            }
        }
    }
}

#[derive(Clone)]
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

pub struct SearchTmResponse {
    pub game_version: String,
    pub list: Pagination<TmEntryPair>,
}

pub struct Pagination<T> {
    pub total: usize,
    pub items: Vec<T>,
}
pub struct TmEntryPair {
    pub key: String,
    pub source: TmEntry,
    pub targets: Vec<TmEntry>,
}

pub struct TmEntry {
    pub language: Language,
    pub content: String,
}

pub struct Upload {
    pub platform: MinecraftPlatform,
    pub language: Language,
    pub game_version: String,
    pub entries: Vec<UploadEntry>,
}

pub struct UploadEntry {
    pub namespace: String,
    pub key: String,
    pub value: String,
}

#[async_trait]
pub trait TmDatabase {
    type Error: std::error::Error + Sync + Send + 'static;

    async fn search(&self, query: SearchTmQuery) -> Result<SearchTmResponse, Self::Error>;

    async fn upload(&self, upload: Upload) -> Result<(), Self::Error>;
}
