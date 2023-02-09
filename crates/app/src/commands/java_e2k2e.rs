use bot_any::types::MessageWrite;
use kal::Command;

use crate::{
    e2k_base::{e2k, k2e, read_lang_file},
    file_reader::AssetStore,
};

/// Minecraft에서 해당 문자열이 포함된 영어 문자열을 검색해 한국어 대응 문자열과 함께 보여줍니다.
#[derive(Command)]
#[command(rename = "javae2k")]
pub struct E2K {
    /// 검색할 문자열
    query: String,

    /// 페이지
    page: Option<i64>,
}

/// Minecraft에서 해당 문자열이 포함된 한국어 문자열을 검색해 영어 대응 문자열과 함께 보여줍니다.
#[derive(Command)]
#[command(rename = "javak2e")]
pub struct K2E {
    /// 검색할 문자열
    query: String,

    /// 페이지
    page: Option<i64>,
}

impl E2K {
    pub async fn execute<'a>(self, asset_store: &AssetStore<'a>) -> eyre::Result<MessageWrite> {
        let en_us = read_lang_file(&asset_store.read_asset("lang/java/en_us.json").await?)?;
        let ko_kr = read_lang_file(&asset_store.read_asset("lang/java/ko_kr.json").await?)?;

        e2k(self.query, self.page, en_us, ko_kr)
    }
}

impl K2E {
    pub async fn execute<'a>(self, asset_store: &AssetStore<'a>) -> eyre::Result<MessageWrite> {
        let en_us = read_lang_file(&asset_store.read_asset("lang/java/en_us.json").await?)?;
        let ko_kr = read_lang_file(&asset_store.read_asset("lang/java/ko_kr.json").await?)?;

        k2e(self.query, self.page, en_us, ko_kr)
    }
}
