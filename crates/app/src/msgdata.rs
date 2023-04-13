use percent_encoding::{percent_decode_str, utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');
const MAGIC: &str = "https://data.store/";

pub fn encode_msgdata(s: &str) -> String {
    format!(
        "[ ]({MAGIC}{})",
        utf8_percent_encode(&s, FRAGMENT).to_string()
    )
}

pub fn decode_msgdata(s: &str) -> Option<String> {
    if let Some(index) = s.find(MAGIC) {
        Some(
            percent_decode_str(&s[index + MAGIC.len()..s.len() - 1])
                .decode_utf8_lossy()
                .to_string(),
        )
    } else {
        None
    }
}
