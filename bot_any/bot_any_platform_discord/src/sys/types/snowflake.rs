use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Hash)]
#[serde(transparent)]
pub struct Snowflake(pub String);
