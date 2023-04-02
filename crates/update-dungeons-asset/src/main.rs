use std::{
    collections::HashMap,
    env,
    fs::{self, File, OpenOptions},
    path::PathBuf,
};

use aes::cipher::KeyInit;
use engine::{
    db::{MinecraftPlatform, PrismaDatabase, TmDatabase, Upload, UploadEntry},
    language::Language,
};
use memmap2::Mmap;
use repak::PakReader;

use crate::locres::LocresFile;
mod locres;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install().ok();

    let database = PrismaDatabase::connect().await?;

    let installation = env::var("DUNGEONS_INSTALLATION")?;
    let version = env::var("DUNGEONS_VERSION")?;
    let key = env::var("DUNGEONS_AES_KEY")?;
    let mut key: &str = &key;
    if &key[0..2] == "0x" {
        key = &key[2..];
    } else {
        key = &key[..];
    }
    let key = key
        .bytes()
        .collect::<Vec<_>>()
        .chunks_exact(2)
        .map(|a| u8::from_str_radix(&String::from_utf8_lossy(a), 16).unwrap())
        .collect::<Vec<_>>();

    let pak_directory =
        PathBuf::from(&installation).join("dungeons/dungeons/Dungeons/Content/Paks");

    let suspicious_asset_names = ["ko-kr/game.locres", "en/game.locres"];

    for chunk in fs::read_dir(&pak_directory)? {
        let pak_path = chunk.unwrap().path();
        let mut file = File::open(&pak_path)?;
        let pak = PakReader::new_any(&mut file, Some(aes::Aes256::new_from_slice(&key)?))?;
        for entry in pak.files() {
            let entry_lower = entry.to_lowercase();
            if !suspicious_asset_names
                .iter()
                .any(|s| entry_lower.contains(s))
            {
                continue;
            }

            println!("{}", entry);

            let mut file = File::open(&pak_path)?;
            let mut writer = OpenOptions::new()
                .create(true)
                .write(true)
                .read(true)
                .open(".tmp")
                .unwrap();
            pak.read_file(&entry, &mut file, &mut writer)?;
            let memmap = unsafe { Mmap::map(&writer) }?;
            let (_, locres) = LocresFile::read(&memmap).map_err(|e| {
                let s = e.to_string();
                eyre::eyre!("{}", &s[s.len() - 1000..])
            })?;

            let language_detected = if entry_lower.contains("ko-kr") {
                Some(Language::Korean)
            } else if entry_lower.contains("en") {
                Some(Language::English)
            } else {
                None
            };

            let mut entries = Vec::new();

            if let Some(language) = language_detected {
                let len = locres.len();
                for (i, ns) in locres.into_values().enumerate() {
                    let namespace = ns.name().to_string();
                    let mut map = HashMap::<String, UploadEntry>::new();
                    println!("{}/{} - {:?}", i + 1, len, ns.name());
                    for (key, value) in ns.into_iter() {
                        map.entry(key.to_lowercase())
                            .and_modify(|old| {
                                if old.value != value {
                                    panic!(
                                        "{} != {} for {} (namespace {})",
                                        old.value, value, key, namespace
                                    );
                                }
                            })
                            .or_insert(UploadEntry {
                                namespace: namespace.clone(),
                                key,
                                value,
                            });
                    }
                    entries.extend(map.into_values());
                }

                database
                    .upload(Upload {
                        platform: MinecraftPlatform::Dungeons,
                        language: language.clone(),
                        game_version: version.clone(),
                        entries,
                    })
                    .await?;
            }
        }
    }

    Ok(())
}
