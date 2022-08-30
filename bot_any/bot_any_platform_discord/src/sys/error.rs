use serde::Deserialize;

#[derive(Deserialize)]
pub struct Error {
    pub code: u64,
    pub message: String,
}

#[derive(Deserialize)]
pub enum DiscordResult<T> {
    Ok(T),
    Err(Error),
}
