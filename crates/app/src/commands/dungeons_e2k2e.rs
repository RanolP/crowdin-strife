use engine::api::{CrowdinStrifeApi, Language, MinecraftPlatform, SourceLanguage};
use kal::Command;

use crate::e2k_base::search_tm;

/// Minecraft에서 해당 문자열이 포함된 영어 문자열을 검색해 한국어 대응 문자열과 함께 보여줍니다.
#[derive(Command)]
#[command(rename = "dgnse2k")]
pub struct E2K {
    /// 검색할 문자열
    query: String,

    /// 페이지
    page: Option<i64>,
}

/// Minecraft에서 해당 문자열이 포함된 한국어 문자열을 검색해 영어 대응 문자열과 함께 보여줍니다.
#[derive(Command)]
#[command(rename = "dgnsk2e")]
pub struct K2E {
    /// 검색할 문자열
    query: String,

    /// 페이지
    page: Option<i64>,
}

impl E2K {
    pub async fn execute(self, api: &dyn CrowdinStrifeApi) -> eyre::Result<String> {
        search_tm(
            api,
            MinecraftPlatform::Dungeons,
            SourceLanguage::Specified(Language::English),
            self.query,
            self.page,
        )
        .await
    }
}

impl K2E {
    pub async fn execute<'a>(self, api: &dyn CrowdinStrifeApi) -> eyre::Result<String> {
        search_tm(
            api,
            MinecraftPlatform::Dungeons,
            SourceLanguage::Specified(Language::Korean),
            self.query,
            self.page,
        )
        .await
    }
}