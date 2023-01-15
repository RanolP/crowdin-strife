use std::{cmp::Ordering, collections::HashMap};

use bot_any::types::MessageWrite;

#[derive(PartialEq, Eq, Ord)]
pub struct Word<'a> {
    pub key: &'a str,
    pub src: &'a str,
    pub dst: &'a str,
}

impl<'a> PartialOrd<Word<'a>> for Word<'a> {
    fn partial_cmp(&self, other: &Word) -> Option<Ordering> {
        self.key.partial_cmp(other.key)
    }
}

pub fn read_lang_file<'a>(src: &'a str) -> eyre::Result<HashMap<String, String>> {
    Ok(serde_json::from_str(src)?)
}

pub fn search<'a>(
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

pub fn e2k(
    query: String,
    page: Option<i64>,
    en_us: HashMap<String, String>,
    ko_kr: HashMap<String, String>,
) -> eyre::Result<MessageWrite> {
    let mut message = MessageWrite::begin();

    let query = query.to_lowercase();
    let page = (page.unwrap_or(1) - 1) as usize;
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

pub fn k2e(
    query: String,
    page: Option<i64>,
    en_us: HashMap<String, String>,
    ko_kr: HashMap<String, String>,
) -> eyre::Result<MessageWrite> {
    let mut message = MessageWrite::begin();

    let query = query.to_lowercase();
    let page = (page.unwrap_or(1) - 1) as usize;
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
