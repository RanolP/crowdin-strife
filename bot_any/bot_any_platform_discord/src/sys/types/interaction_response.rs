use serde::Serialize;
use serde_repr::Serialize_repr;

use super::InteractionCallbackMessage;

#[derive(Serialize_repr)]
#[repr(u32)]
pub enum InteractionVCallbackKind {
    Pong = 1,
    ChannelMessageWithSource = 4,
    DeferredChannelMessageWithSource = 5,
    DeferredUpdateMessage = 6,
    UpdateMessage = 7,
    ApplicationCommandAutoCompleteResult = 8,
    Modal = 9,
}

#[derive(Serialize)]
#[serde(untagged)]
pub enum InteractionCallbackData {
    Message(InteractionCallbackMessage),
}

#[derive(Serialize)]
pub struct InteractionResponse {
    #[serde(rename = "type")]
    kind: InteractionVCallbackKind,
    data: Option<InteractionCallbackData>,
}

impl InteractionResponse {
    pub fn pong() -> Self {
        InteractionResponse {
            kind: InteractionVCallbackKind::Pong,
            data: None,
        }
    }

    pub fn message_with_source(message: InteractionCallbackMessage) -> Self {
        InteractionResponse {
            kind: InteractionVCallbackKind::ChannelMessageWithSource,
            data: Some(InteractionCallbackData::Message(message)),
        }
    }
}
