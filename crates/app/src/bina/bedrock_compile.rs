use std::{collections::HashMap, env, fs::File, io::Write};

use crowdin_strife::e2k_base::search;
use encoding::codec::utf_8::UTF8Encoding;

fn main() {
    const EN_US: &[u8] = include_bytes!("../../assets/lang/bedrock/en_US.lang");
    const KO_KR: &[u8] = include_bytes!("../../assets/lang/bedrock/ko_KR.lang");

    let assets_dir = env::current_dir().unwrap().join("assets/lang/bedrock");

    File::create(assets_dir.join("en_us.json"))
        .unwrap()
        .write(
            &serde_json::to_vec(
                &java_properties::PropertiesIter::new_with_encoding(EN_US, &UTF8Encoding)
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
                    .collect::<HashMap<_, _>>(),
            )
            .unwrap(),
        )
        .unwrap();
    File::create(assets_dir.join("ko_kr.json"))
        .unwrap()
        .write(
            &serde_json::to_vec(
                &java_properties::PropertiesIter::new_with_encoding(KO_KR, &UTF8Encoding)
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
                    .collect::<HashMap<_, _>>(),
            )
            .unwrap(),
        )
        .unwrap();
}
