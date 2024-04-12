use percent_encoding::{percent_decode_str, utf8_percent_encode, AsciiSet, CONTROLS};

const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"').add(b'<').add(b'>').add(b'`');
const MAGIC: &str = "https://data.store/";

pub fn encode_msgdata(s: &str) -> String {
    format!("[\u{2800}]({MAGIC}{})", utf8_percent_encode(s, FRAGMENT))
}

pub fn decode_msgdata(s: &str) -> Option<String> {
    s.find(MAGIC).map(|index| {
        percent_decode_str(&s[index + MAGIC.len()..s.len() - 1])
            .decode_utf8_lossy()
            .to_string()
    })
}
