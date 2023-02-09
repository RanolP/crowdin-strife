pub struct AssetStore<'a>(pub &'a worker::Env);
impl AssetStore<'_> {
    pub async fn read_asset(&self, path: &str) -> eyre::Result<String> {
        let kv = self
            .0
            .kv("__STATIC_CONTENT")
            .map_err(|e| eyre::eyre!("{}", e))?;
        let got = if let Some(ext_dot) = path.rfind(".") {
            let alt_key = kv
                .list()
                .prefix(path[..ext_dot].to_string())
                .execute()
                .await
                .map_err(|e| eyre::eyre!("{}", e))?
                .keys
                .into_iter()
                .find(|k| k.name.ends_with(&path[ext_dot..]))
                .ok_or_else(|| eyre::eyre!("No Entry: {}", path))?;
            kv.get(&alt_key.name)
        } else {
            kv.get(path)
        };
        got.text()
            .await
            .map_err(|e| eyre::eyre!("{}", e))?
            .ok_or_else(|| eyre::eyre!("No Entry: {}", path))
    }
}
