use once_cell::sync::Lazy;
use regex::Regex;

use super::Language;

static HANGUL: Lazy<Regex> = Lazy::new(|| Regex::new(r#"[가-힣ㄱ-ㅎㅏ-ㅣ]"#).unwrap());

impl Language {
    pub fn infer_from_text(text: &str) -> Vec<Language> {
        if HANGUL.is_match(text) {
            vec![Language::Korean]
        } else {
            vec![]
        }
    }
}
