use serde::Serialize;

use super::Embed;

#[derive(Serialize)]
pub struct InteractionCallbackMessage {
    pub tts: Option<bool>,
    pub content: Option<String>,
    pub embeds: Vec<Embed>,
    // components?	array of components	message components
    // attachments? *	array of partial attachment objects	attachment objects with filename and description
}
