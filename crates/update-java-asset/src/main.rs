#![cfg(not(target_arch = "wasm32"))]

use std::{collections::HashMap, io::Cursor, time::Instant};

use engine::db::{Language, MinecraftPlatform, PrismaDatabase, TmDatabase, Upload, UploadWord};
use mcapi::launcher::{
    AssetBundle, DownloadAsset, DownloadGame, GetAssetBundle, GetAssetIndex, GetVersionManifest,
    Version,
};
use reqores_client_surf::SurfClient;
use zip::ZipArchive;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    color_eyre::install().ok();

    let client = SurfClient::new();
    let database = PrismaDatabase::connect().await?;

    let (version, bundle) = fetch_snapshot(&client).await?;
    let en_us = fetch_en_us(&client, &bundle).await?;

    let now = Instant::now();
    database
        .upload(Upload {
            platform: MinecraftPlatform::Java,
            filename: "assets/minecraft/lang/en_us.json".to_string(),
            language: Language::English,
            game_version: version.id.clone(),
            words: en_us
                .into_iter()
                .map(|(key, value)| UploadWord { key, value })
                .collect(),
        })
        .await?;
    println!(
        "Upload en_us.json took {:.1}s",
        Instant::now().duration_since(now).as_secs_f32()
    );

    let now = Instant::now();
    let ko_kr = fetch_ko_kr(&client, &bundle).await?;
    database
        .upload(Upload {
            platform: MinecraftPlatform::Java,
            filename: "assets/minecraft/lang/ko_kr.json".to_string(),
            language: Language::Korean,
            game_version: version.id.clone(),
            words: ko_kr
                .into_iter()
                .map(|(key, value)| UploadWord { key, value })
                .collect(),
        })
        .await?;
    println!(
        "Upload ko_kr.json took {:.1}s",
        Instant::now().duration_since(now).as_secs_f32()
    );

    println!("Done!");

    Ok(())
}

async fn fetch_snapshot(client: &SurfClient) -> eyre::Result<(Version, AssetBundle)> {
    let version_manifest = client
        .call(GetVersionManifest)
        .await
        .map_err(|e| eyre::eyre!("{}", e))?;

    let latest_snapshot = version_manifest.latest.snapshot;
    println!(
        "Fetched version manifest, latest snapshot is {}",
        latest_snapshot
    );

    let latest_snapshot = version_manifest
        .versions
        .into_iter()
        .find(|version| version.id == latest_snapshot)
        .ok_or(eyre::eyre!("Latest snapshot cannot be found"))?;

    let asset_bundle = client
        .call(GetAssetBundle {
            version: &latest_snapshot,
        })
        .await
        .map_err(|e| eyre::eyre!("{}", e))?;

    Ok((latest_snapshot, asset_bundle))
}

async fn fetch_en_us(
    client: &SurfClient,
    asset_bundle: &AssetBundle,
) -> eyre::Result<HashMap<String, String>> {
    println!("Fetched asset bundle for latest snapshot");

    let now = Instant::now();
    println!(
        "Downloading game client from {}",
        asset_bundle.downloads.client.url
    );
    let game_client = client
        .call(DownloadGame {
            game_file: &asset_bundle.downloads.client,
        })
        .await
        .map_err(|e| eyre::eyre!("{}", e))?;
    println!(
        "Downloaded game client in {:.1}s",
        Instant::now().duration_since(now).as_secs_f32()
    );

    let mut game_client_zip = ZipArchive::new(Cursor::new(game_client))?;
    let en_us_file = game_client_zip.by_name("assets/minecraft/lang/en_us.json")?;

    let en_us = serde_json::from_reader(en_us_file)?;

    Ok(en_us)
}

async fn fetch_ko_kr(
    client: &SurfClient,
    bundle: &AssetBundle,
) -> eyre::Result<HashMap<String, String>> {
    let asset_index = client
        .call(GetAssetIndex { bundle: &bundle })
        .await
        .map_err(|e| eyre::eyre!("{}", e))?;

    println!("Fetched asset index for latest snapshot");

    let ko_kr = asset_index
        .objects
        .get("minecraft/lang/ko_kr.json")
        .ok_or(eyre::eyre!("ko_kr.json not found"))?;

    let ko_kr = client
        .call(DownloadAsset { asset: ko_kr })
        .await
        .map_err(|e| eyre::eyre!("{}", e))?;

    let ko_kr = serde_json::from_slice(&ko_kr)?;

    Ok(ko_kr)
}
