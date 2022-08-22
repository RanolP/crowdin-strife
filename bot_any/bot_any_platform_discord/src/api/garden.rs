use reqores::{ServerRequest, ServerResponse, ServerResponseBuilder, StatusCode};
use thiserror::Error;

use crate::sys::{
    types::{ApplicationCommand, Interaction, InteractionResponse, RawInteraction},
    verify_key::{VerifyKey, VerifyKeyError},
};

pub enum DiscordPlant {
    EarlyReturn,
    Command(ApplicationCommand),
}

pub struct DiscordGarden {
    verify_key: VerifyKey,
}

#[derive(Debug, Error)]
pub enum DiscordGardenError {
    #[error("Failed to verify with public key: {0}")]
    VerifyKey(#[from] VerifyKeyError),
    #[error("Failed to deserialize body: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

impl DiscordGarden {
    pub fn new(client_public_key: &str) -> Result<Self, VerifyKeyError> {
        Ok(DiscordGarden {
            verify_key: VerifyKey::new(client_public_key)?,
        })
    }

    pub async fn accept(
        &self,
        req: &impl ServerRequest,
    ) -> Result<(ServerResponse, DiscordPlant), DiscordGardenError> {
        let response = self.verify_key.accept(req).await?;
        let interaction: RawInteraction = req.body_json()?;
        let response = match interaction.transform() {
            Some(Interaction::Ping) => (
                response.then(
                    ServerResponseBuilder::new()
                        .status(StatusCode::Ok)
                        .body_json(&InteractionResponse::pong())?,
                ),
                DiscordPlant::EarlyReturn,
            ),
            Some(Interaction::ApplicationCommand(command)) => {
                (response, DiscordPlant::Command(command))
            }
            None => (
                ServerResponseBuilder::new()
                    .status(StatusCode::BadRequest)
                    .body_str("Server failed to decode the interaction"),
                DiscordPlant::EarlyReturn,
            ),
        };
        Ok(response)
    }
}
