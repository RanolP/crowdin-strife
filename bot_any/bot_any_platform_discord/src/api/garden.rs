use reqores::{HttpStatusCode, ServerRequest, ServerResponse, ServerResponseBuilder};
use thiserror::Error;

use crate::sys::{
    types::{Interaction, InteractionApplicationCommand, InteractionResponse, RawInteraction},
    verify_key::{VerifyKey, VerifyKeyError},
};

pub enum DiscordPlant {
    EarlyReturn,
    Command(InteractionApplicationCommand),
}

pub struct DiscordGarden {
    verify_key: Option<VerifyKey>,
}

#[derive(Debug, Error)]
pub enum DiscordGardenError {
    #[error("Failed to verify with public key: {0}")]
    VerifyKey(#[from] VerifyKeyError),
    #[error("Failed to deserialize body: {0}")]
    SerdeJson(#[from] serde_json::Error),
}

impl DiscordGarden {
    pub fn new(client_public_key: Option<&str>) -> Result<Self, VerifyKeyError> {
        Ok(DiscordGarden {
            verify_key: client_public_key.map(VerifyKey::new).transpose()?,
        })
    }

    pub async fn seed(
        &self,
        req: &impl ServerRequest,
    ) -> Result<(ServerResponse, DiscordPlant), DiscordGardenError> {
        let response = if let Some(verify_key) = &self.verify_key {
            verify_key.accept(req).await?
        } else {
            ServerResponseBuilder::new().build()
        };
        let interaction: RawInteraction = req.body_json()?;
        let response = match interaction.transform() {
            Some(Interaction::Ping) => (
                response.then(
                    ServerResponseBuilder::new()
                        .status(HttpStatusCode::Ok)
                        .body_json(&InteractionResponse::pong())?,
                ),
                DiscordPlant::EarlyReturn,
            ),
            Some(Interaction::ApplicationCommand(command)) => {
                (response, DiscordPlant::Command(command))
            }
            None => (
                ServerResponseBuilder::new()
                    .status(HttpStatusCode::BadRequest)
                    .body_str("Server failed to decode the interaction"),
                DiscordPlant::EarlyReturn,
            ),
        };
        Ok(response)
    }
}
