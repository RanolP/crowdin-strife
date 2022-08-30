#![cfg(not(target_arch = "wasm32"))]

use std::collections::HashMap;

use mcapi::launcher::{DownloadAsset, GetAssetBundle, GetAssetIndex, GetVersionManifest};
use reqores_client_surf::SurfClient;

#[tokio::main]
async fn main() -> eyre::Result<()> {
    dotenvy::dotenv().ok();
    color_eyre::install().ok();

    let client = SurfClient;

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
        .iter()
        .find(|version| version.id == latest_snapshot)
        .ok_or(eyre::eyre!("Latest snapshot cannot be found"))?;

    let asset_bundle = client
        .call(GetAssetBundle {
            version: latest_snapshot,
        })
        .await
        .map_err(|e| eyre::eyre!("{}", e))?;

    println!("Fetched asset bundle for latest snapshot");

    let asset_index = client
        .call(GetAssetIndex {
            bundle: &asset_bundle,
        })
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

    let ko_kr: HashMap<String, String> = serde_json::from_slice(&ko_kr)?;

    println!("{:?}", ko_kr.iter().find(|_| true));

    Ok(())
}
