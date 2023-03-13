use std::collections::HashMap;

use encoding::codec::utf_8::UTF8Encoding;

fn main() {
    const EN_US: &[u8] = include_bytes!("../../../assets/lang/bedrock/en_US.lang");
    const KO_KR: &[u8] = include_bytes!("../../../assets/lang/bedrock/ko_KR.lang");

    fn read_lang_file(src: &[u8]) -> HashMap<String, String> {
        java_properties::PropertiesIter::new_with_encoding(src, &UTF8Encoding)
            .flat_map(|r| r.ok())
            .flat_map(|l| match l.consume_content() {
                java_properties::LineContent::Comment(_) => None,
                java_properties::LineContent::KVPair(k, v) => {
                    if let Some(i) = v.find("\t#") {
                        Some((k, v[..i].to_string()))
                    } else {
                        Some((k, v))
                    }
                }
            })
            .collect::<HashMap<_, _>>()
    }

    let en_us = read_lang_file(EN_US);
    let ko_kr = read_lang_file(KO_KR);
}
