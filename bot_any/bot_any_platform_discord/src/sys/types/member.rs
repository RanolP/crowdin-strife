use serde::Deserialize;

use super::User;

#[derive(Deserialize)]
pub struct Member {
    pub user: User,
    pub nick: Option<String>,
}

impl From<Member> for bot_any::types::User {
    fn from(member: Member) -> Self {
        bot_any::types::User {
            id: member.user.id.0,
            display_name: member.nick.unwrap_or(member.user.username),
        }
    }
}

#[derive(Deserialize)]
pub struct PartialMember {}
