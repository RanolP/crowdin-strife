use bot_any::types::User as BotanyUser;

use crate::sys::types::User;

impl From<User> for BotanyUser {
    fn from(user: User) -> Self {
        BotanyUser {
            id: user.id.0,
            display_name: user.username,
        }
    }
}
