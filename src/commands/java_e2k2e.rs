use std::{cmp::Ordering, collections::HashMap};

use bot_any::types::MessageWrite;
use kal::Command;

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

#[derive(PartialEq, Eq, Ord)]
struct Word<'a> {
    key: &'a str,
    src: &'a str,
    dst: &'a str,
}

impl<'a> PartialOrd<Word<'a>> for Word<'a> {
    fn partial_cmp(&self, other: &Word) -> Option<Ordering> {
        self.key.partial_cmp(other.key)
    }
}

fn search<'a>(
    src_map: &'a HashMap<String, String>,
    dst_map: &'a HashMap<String, String>,
    query: &'a str,
    page: usize,
) -> (Vec<Word<'a>>, usize) {
    let mut words_found: Vec<_> = src_map
        .into_iter()
        .filter(|(_, value)| value.to_lowercase().contains(query))
        .map(|(key, src)| Word {
            key,
            src,
            dst: dst_map.get(key).map_or("*대응어 없음*", String::as_ref),
        })
        .collect();
    words_found.sort();
    let total_len = words_found.len();
    (
        words_found.into_iter().skip(page * 10).take(10).collect(),
        (total_len + 9) / 10,
    )
}

fn read_lang_file<'a>(src: &'a str) -> eyre::Result<HashMap<String, String>> {
    Ok(serde_json::from_str(src)?)
}

const EN_US: &str = include_str!("../../assets/lang/java/en_us.json");
const KO_KR: &str = include_str!("../../assets/lang/java/ko_kr.json");

impl E2K {
    pub async fn execute(self) -> eyre::Result<MessageWrite> {
        let en_us = read_lang_file(EN_US)?;
        let ko_kr = read_lang_file(KO_KR)?;

        let mut message = MessageWrite::begin();

        let query = self.query.to_lowercase();
        let page = (self.page.unwrap_or(1) - 1) as usize;
        let (found, total_pages) = search(&en_us, &ko_kr, &query, page);
        for word in found {
            message = message.push_str(format!("{} => {}\n", word.src, word.dst));
        }

        if message.is_empty() {
            message = message.push_str("결과 없음".to_string());
        } else if total_pages > 1 {
            message = message.push_str(format!("페이지 {} / {}", page + 1, total_pages));
        }

        Ok(message.end())
    }
}

impl K2E {
    pub async fn execute(self) -> eyre::Result<MessageWrite> {
        let en_us = read_lang_file(EN_US)?;
        let ko_kr = read_lang_file(KO_KR)?;

        let mut message = MessageWrite::begin();

        let query = self.query.to_lowercase();
        let page = (self.page.unwrap_or(1) - 1) as usize;
        let (found, total_pages) = search(&ko_kr, &en_us, &query, page);
        for word in found {
            message = message.push_str(format!("{} => {}\n", word.src, word.dst));
        }

        if message.is_empty() {
            message = message.push_str("결과 없음".to_string());
        } else if total_pages > 1 {
            message = message.push_str(format!("페이지 {} / {}", page + 1, total_pages));
        }

        Ok(message.end())
    }
}
