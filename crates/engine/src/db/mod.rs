use std::future::Future;

pub use r#impl::*;

use crate::language::Language;

mod r#impl;

#[derive(Clone)]
pub enum MinecraftPlatform {
    Java,
    Bedrock,
    Dungeons,
}

impl MinecraftPlatform {
    pub fn id(&self) -> &'static str {
        match self {
            MinecraftPlatform::Java => "Java",
            MinecraftPlatform::Bedrock => "Bedrock",
            MinecraftPlatform::Dungeons => "Dungeons",
        }
    }

    pub fn from_id(s: &str) -> Option<MinecraftPlatform> {
        match s {
            "Java" => Some(MinecraftPlatform::Java),
            "Bedrock" => Some(MinecraftPlatform::Bedrock),
            "Dungeons" => Some(MinecraftPlatform::Dungeons),
            _ => None,
        }
    }
}

pub struct SearchTmQuery {
    pub source: Language,
    pub target: Language,
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

pub trait TmDatabase {
    type Error: std::error::Error + Sync + Send + 'static;

    fn search(
        &self,
        query: SearchTmQuery,
    ) -> impl Future<Output = Result<SearchTmResponse, Self::Error>> + Send;

    fn upload(&self, upload: Upload) -> impl Future<Output = Result<(), Self::Error>> + Send;
}
