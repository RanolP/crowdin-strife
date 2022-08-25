use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize, Hash)]
#[serde(transparent)]
pub struct Snowflake(pub String);
