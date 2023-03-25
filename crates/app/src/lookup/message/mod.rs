use engine::db::Language;
use serenity::builder::CreateInteractionResponseData;

use crate::message::StructuredMessage;

pub struct LookupResult {
    query: String,

    source_language: Language,
    target_language: Language,

    game_version: String,
}
