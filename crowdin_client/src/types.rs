use serde::{Deserialize, Serialize};
use serde_with::{serde_as, DisplayFromStr};

#[derive(Serialize, Deserialize)]
pub struct UserId(pub String);

#[serde_as]
#[derive(Serialize, Deserialize, Clone, PartialEq)]
#[serde(transparent)]
pub struct LanguageId(#[serde_as(as = "DisplayFromStr")] pub u32);

impl LanguageId {
    pub const KOREAN: LanguageId = LanguageId(27);
}

#[derive(Serialize, Deserialize)]
pub struct CrowdinResponse<T> {
    success: bool,
    data: Either<Error, T>,
    version: String,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Either<L, R> {
    Left(L),
    Right(R),
}

#[derive(Serialize, Deserialize)]
pub struct Error {
    error: bool,
}
