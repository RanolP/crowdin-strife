use serde::Serialize;

use super::Embed;

#[derive(Serialize)]
pub struct InteractionCallbackMessage {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tts: Option<bool>,
    pub content: Option<String>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub embeds: Vec<Embed>,
    // components?	array of components	message components
    // attachments? *	array of partial attachment objects	attachment objects with filename and description
}
