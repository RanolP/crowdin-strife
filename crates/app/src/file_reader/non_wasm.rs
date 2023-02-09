use std::{env, fs::File, io::Read, marker::PhantomData};

pub struct AssetStore<'a>(PhantomData<&'a ()>);

impl AssetStore<'_> {
    pub async fn read_asset(&self, path: &str) -> eyre::Result<String> {
        let assets_dir = env::current_dir()?.join("assets/");

        let mut s = String::new();
        File::open(assets_dir.join(path))?.read_to_string(&mut s)?;

        Ok(s)
    }
}
