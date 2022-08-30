use bot_any::types::{MessagePart, MessageWrite};

use crate::sys::types::InteractionCallbackMessage;

impl From<MessageWrite> for InteractionCallbackMessage {
    fn from(payload: MessageWrite) -> Self {
        InteractionCallbackMessage {
            tts: None,
            content: Some(
                payload
                    .contents()
                    .iter()
                    .map(|part| match part {
                        MessagePart::Text(content) => content.clone(),
                    })
                    .collect::<String>(),
            ),
            embeds: vec![],
        }
    }
}
