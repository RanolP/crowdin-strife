#![cfg(not(target_arch = "wasm32"))]

use std::{
    env,
    fs::{self, File},
    io::{Cursor, Write},
    time::Instant,
};

use log::info;
use mcapi::launcher::{
    DownloadAsset, DownloadGame, GetAssetBundle, GetAssetIndex, GetVersionManifest,
};
use reqores_client_surf::SurfClient;
use surf::{middleware::Logger, Client};
use zip::ZipArchive;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenvy::dotenv().ok();
    color_eyre::install().ok();
    pretty_env_logger::try_init()?;

    let client = SurfClient::with_client(Client::new().with(Logger::new()));

    let version_manifest = client
        .call(GetVersionManifest)
        .await
        .map_err(|e| eyre::eyre!("{}", e))?;

    let latest_snapshot = version_manifest.latest.snapshot;
    info!(
        "Fetched version manifest, latest snapshot is {}",
        latest_snapshot
    );

    let latest_snapshot = version_manifest
        .versions
        .iter()
        .find(|version| version.id == latest_snapshot)
        .ok_or(eyre::eyre!("Latest snapshot cannot be found"))?;

    let asset_bundle = client
        .call(GetAssetBundle {
            version: latest_snapshot,
        })
        .await
        .map_err(|e| eyre::eyre!("{}", e))?;

    info!("Fetched asset bundle for latest snapshot");

    let now = Instant::now();
    info!(
        "Downloading game client from {}",
        asset_bundle.downloads.client.url
    );
    let game_client = client
        .call(DownloadGame {
            game_file: &asset_bundle.downloads.client,
        })
        .await
        .map_err(|e| eyre::eyre!("{}", e))?;
    info!(
        "Downloaded game client in {:.1}s",
        Instant::now().duration_since(now).as_secs_f32()
    );

    let mut game_client_zip = ZipArchive::new(Cursor::new(game_client))?;
    let en_us_file = game_client_zip.by_name("assets/minecraft/lang/en_us.json")?;

    let en_us = serde_json::from_reader::<_, serde_json::Value>(en_us_file)
        .and_then(|v| serde_json::to_vec(&v))
        .map_err(|e| eyre::eyre!("{}", e))?;

    let asset_index = client
        .call(GetAssetIndex {
            bundle: &asset_bundle,
        })
        .await
        .map_err(|e| eyre::eyre!("{}", e))?;

    info!("Fetched asset index for latest snapshot");

    let ko_kr = asset_index
        .objects
        .get("minecraft/lang/ko_kr.json")
        .ok_or(eyre::eyre!("ko_kr.json not found"))?;

    let ko_kr = client
        .call(DownloadAsset { asset: ko_kr })
        .await
        .map_err(|e| eyre::eyre!("{}", e))?;

    let ko_kr = serde_json::from_slice::<serde_json::Value>(&ko_kr)
        .and_then(|v| serde_json::to_vec(&v))
        .map_err(|e| eyre::eyre!("{}", e))?;

    let assets_dir = env::current_dir()?.join("assets/lang/java");
    if !assets_dir.exists() {
        fs::create_dir_all(&assets_dir)?;
    }

    File::create(assets_dir.join("ko_kr.json"))?.write(&ko_kr)?;
    File::create(assets_dir.join("en_us.json"))?.write(&en_us)?;

    Ok(())
}
