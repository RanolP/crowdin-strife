use std::{
    collections::HashMap,
    env, fs,
    io::{Cursor, Read},
    path::PathBuf,
    time::Instant,
};

use encoding::codec::utf_8::UTF8Encoding;
use engine::{
    db::{MinecraftPlatform, PrismaDatabase, TmDatabase, Upload, UploadEntry},
    language::Language,
};
use eyre::bail;

pub struct WindowsApp {
    id: &'static str,
    publisher_id: &'static str,
}

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install().ok();

    let database = PrismaDatabase::connect().await?;

    let installation = env::var("BEDROCK_INSTALLATION")?;

    let minecraft = WindowsApp {
        id: "Microsoft.MinecraftUWP",
        publisher_id: "8wekyb3d8bbwe",
    };
    let minecraft_preview = WindowsApp {
        id: "Microsoft.MinecraftWindowsBeta",
        publisher_id: "8wekyb3d8bbwe",
    };

    // later is preferred
    let prioritized_minecraft_apps = vec![minecraft, minecraft_preview];

    let mut found = vec![None; prioritized_minecraft_apps.len()];

    for folder in fs::read_dir(PathBuf::from(&installation))? {
        let folder = folder?;
        let path = folder.path();
        let filename = folder.file_name();
        let filename = filename.to_string_lossy().to_string();
        for (id, app) in prioritized_minecraft_apps.iter().enumerate() {
            if filename.contains(app.id) {
                let version = filename
                    .replace(&format!("{}_", app.id), "")
                    .replace(&format!("__{}", app.publisher_id), "")
                    .replace("_x64", "")
                    .replace("_x86", "");
                let version_segment = version.split('.').collect::<Vec<_>>();
                let major = version_segment[0];
                let minor = version_segment[1];
                let mut patch = version_segment[2];
                if patch.len() > 2 {
                    patch = &patch[..2];
                }
                let version = format!("{major}.{minor}.{patch}");

                found[id] = Some((path, filename, version));
                break;
            }
        }
    }

    let Some((folder, filename, version)) = found.into_iter().flatten().last() else {
        bail!("Cannot found installation of Microsoft.MinecraftUWP")
    };

    println!("{filename}");
    println!("version: {version}");

    let resource_pack = folder.join("data/resource_packs");

    let mut english = Vec::new();
    let mut korean = Vec::new();

    for entry in fs::read_dir(resource_pack)? {
        let entry = entry?;

        let namespace = entry.file_name();
        let namespace = namespace.to_string_lossy().to_string();

        let texts = entry.path().join("texts");
        let en_us = texts.join("en_US.lang");
        let ko_kr = texts.join("ko_KR.lang");
        if !texts.exists() || !ko_kr.exists() || !en_us.exists() {
            println!("❎ {namespace}: no lang files");
            continue;
        }
        println!("✏️ {namespace}: found both texts/ko_KR.lang and texts/en_US.lang");

        let en_us = fs::read_to_string(en_us)?;
        let en_us = read_lang_file(Cursor::new(en_us))?;
        for (key, value) in en_us {
            english.push(UploadEntry {
                namespace: namespace.clone(),
                key,
                value,
            });
        }

        let ko_kr = fs::read_to_string(ko_kr)?;
        let ko_kr = read_lang_file(Cursor::new(ko_kr))?;
        for (key, value) in ko_kr {
            korean.push(UploadEntry {
                namespace: namespace.clone(),
                key,
                value,
            });
        }
    }

    let now = Instant::now();
    println!("🚀 Start uploading!");
    database
        .upload(Upload {
            platform: MinecraftPlatform::Bedrock,
            language: Language::English,
            game_version: version.clone(),
            entries: english,
        })
        .await?;

    database
        .upload(Upload {
            platform: MinecraftPlatform::Bedrock,
            language: Language::Korean,
            game_version: version.clone(),
            entries: korean,
        })
        .await?;
    println!(
        "⏰ Took {:.1}s",
        Instant::now().duration_since(now).as_secs_f32()
    );

    Ok(())
}

fn read_lang_file<R: Read>(src: R) -> eyre::Result<HashMap<String, String>> {
    let lines = java_properties::PropertiesIter::new_with_encoding(src, &UTF8Encoding)
        .collect::<Result<Vec<_>, _>>()?;
    Ok(lines
        .into_iter()
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
        .collect::<HashMap<_, _>>())
}
