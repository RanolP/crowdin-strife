use serde::Deserialize;

#[derive(Deserialize)]
pub struct Error {
    code: u64,
    message: String,
}

#[derive(Deserialize)]
pub enum DiscordResult<T> {
    Ok(T),
    Err(Error),
}
