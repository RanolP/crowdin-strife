use serde::Deserialize;

use super::Snowflake;

#[derive(Deserialize)]
pub struct User {
    pub id: Snowflake,
    pub username: String,
    pub discriminator: String,
}
