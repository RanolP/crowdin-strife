use bot_any::types::User as BotanyUser;

use crate::sys::types::Member;

impl From<Member> for BotanyUser {
    fn from(member: Member) -> Self {
        BotanyUser {
            id: member.user.id.0,
            display_name: member.nick.unwrap_or(member.user.username),
        }
    }
}
