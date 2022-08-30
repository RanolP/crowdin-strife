use serde::Deserialize;

use super::User;

#[derive(Deserialize)]
pub struct Member {
    pub user: User,
    pub nick: Option<String>,
}

#[derive(Deserialize)]
pub struct PartialMember {}
