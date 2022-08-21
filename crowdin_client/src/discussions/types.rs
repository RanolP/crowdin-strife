use std::str::FromStr;

use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Serialize, Deserialize)]
#[serde(transparent)]
pub struct DiscussionId(pub String);

pub enum DiscussionStatus {
    Open,
    Closed,
}

#[derive(Debug, Error)]
#[error("Discussion status must be `open` or `closed` but received {0}")]
pub struct DiscussionStatusParseError(String);

impl FromStr for DiscussionStatus {
    type Err = DiscussionStatusParseError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "open" => Ok(DiscussionStatus::Open),
            "closed" => Ok(DiscussionStatus::Closed),
            _ => Err(DiscussionStatusParseError(s.to_string())),
        }
    }
}

impl DiscussionStatus {
    pub fn as_str(&self) -> &str {
        match self {
            DiscussionStatus::Open => "open",
            DiscussionStatus::Closed => "closed",
        }
    }
}
