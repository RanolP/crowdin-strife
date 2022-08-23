use serde::Deserialize;

use super::Snowflake;

#[derive(Deserialize)]
pub struct User {
    pub id: Snowflake,
    pub username: String,
    pub discriminator: String,
}

impl From<User> for bot_any::types::User {
    fn from(user: User) -> Self {
        bot_any::types::User {
            id: user.id.0,
            display_name: user.username,
        }
    }
}
