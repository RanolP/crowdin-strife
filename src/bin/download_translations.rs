#![cfg(not(target_arch = "wasm32"))]

use mcapi::launcher::{GetAssetBundle, GetVersionManifest};
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

    println!("{:?}", asset_bundle);

    Ok(())
}
