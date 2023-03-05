use std::{collections::HashMap, env, fs::File, io::Write};

use crowdin_strife::e2k_base::search;
use encoding::codec::utf_8::UTF8Encoding;

fn main() {
    const EN_US: &str = include_str!("../../assets/lang/dungeons/en.csv");
    const KO_KR: &str = include_str!("../../assets/lang/dungeons/ko_kr.csv");

    let assets_dir = env::current_dir().unwrap().join("assets/lang/dungeons");

    fn read_lang_file<'a>(src: &'a str) -> eyre::Result<HashMap<String, String>> {
        csv::ReaderBuilder::new()
            .from_reader(src.as_bytes())
            .into_deserialize()
            .collect::<Result<_, _>>()
            .map_err(|e| eyre::eyre!("Failed to read lang file: {}", e))
    }

    File::create(assets_dir.join("en_us.json"))
        .unwrap()
        .write(&serde_json::to_vec(&read_lang_file(EN_US).unwrap()).unwrap())
        .unwrap();
    File::create(assets_dir.join("ko_kr.json"))
        .unwrap()
        .write(&serde_json::to_vec(&read_lang_file(KO_KR).unwrap()).unwrap())
        .unwrap();
}
