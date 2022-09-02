use std::{collections::HashMap, env, fs::File, path::PathBuf};

use bot_any::types::MessageWrite;
use kal::Command;

#[derive(Command)]
#[command(
    name = "e2k",
    description = "해당 문자열이 포함된 영어 문자열을 검색해 한국어 대응 문자열과 함께 보여줍니다."
)]
pub struct E2K {
    #[argument(name = "query", description = "검색할 문자열")]
    query: String,
}

#[derive(Command)]
#[command(
    name = "k2e",
    description = "해당 문자열이 포함된 한국어 문자열을 검색해 영어 대응 문자열과 함께 보여줍니다."
)]
pub struct K2E {
    #[argument(name = "query", description = "검색할 문자열")]
    query: String,
}

fn search(map: HashMap<String, String>, query: &str) -> Vec<(String, String)> {
    map.into_iter()
        .filter(|(_, value)| value.contains(query))
        .collect()
}

fn read_lang_file(path: PathBuf) -> eyre::Result<HashMap<String, String>> {
    Ok(serde_json::from_reader(File::open(path)?)?)
}

impl E2K {
    pub async fn execute(self) -> eyre::Result<MessageWrite> {
        let en_us = read_lang_file(env::current_dir()?.join("assets/lang/en_us.json"))?;
        let ko_kr = read_lang_file(env::current_dir()?.join("assets/lang/ko_kr.json"))?;

        let mut w = MessageWrite::begin();

        for (k, v) in search(en_us, &self.query) {
            let correspondent = ko_kr.get(&k).map_or("*대응어 없음*", String::as_ref);
            w = w.push_str(format!("{} => {}\n", v, correspondent));
        }

        Ok(w.end())
    }
}

impl K2E {
    pub async fn execute(self) -> eyre::Result<MessageWrite> {
        let en_us = read_lang_file(env::current_dir()?.join("assets/lang/en_us.json"))?;
        let ko_kr = read_lang_file(env::current_dir()?.join("assets/lang/ko_kr.json"))?;

        let mut w = MessageWrite::begin();

        for (k, v) in search(ko_kr, &self.query) {
            let correspondent = en_us.get(&k).map_or("*대응어 없음*", String::as_ref);
            w = w.push_str(format!("{} => {}\n", v, correspondent));
        }

        Ok(w.end())
    }
}
