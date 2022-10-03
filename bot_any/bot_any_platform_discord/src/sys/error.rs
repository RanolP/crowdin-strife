use serde::Deserialize;

#[derive(Deserialize)]
#[serde(untagged)]
pub enum DiscordError {
    Coded {
        code: u64,
        message: String,
    },
    RateLimited {
        global: bool,
        message: String,
        retry_after: f64,
    },
    Unknown(serde_json::Value),
}

#[derive(Deserialize)]
#[serde(untagged)]
pub enum DiscordResult<T> {
    Ok(T),
    Err(DiscordError),
}
