const MAGIC: &str = "https://data.store/";

pub fn encode_msgdata(s: String) -> String {
    format!("[]({MAGIC}{s})")
}

pub fn decode_msgdata(s: &str) -> Option<String> {
    if let Some(index) = s.find(MAGIC) {
        Some(s[index + MAGIC.len()..s.len() - 1].to_string())
    } else {
        None
    }
}
