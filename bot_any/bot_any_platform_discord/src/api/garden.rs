use reqores::{ServerRequest, ServerResponse, ServerResponseBuilder, StatusCode};

use crate::sys::{
    types::{Interaction, InteractionResponse, RawInteraction},
    verify_key::{VerifyKey, VerifyKeyError},
};

pub struct DiscordGarden {
    verify_key: VerifyKey,
}

impl DiscordGarden {
    pub fn new(client_public_key: &str) -> Result<Self, VerifyKeyError> {
        Ok(DiscordGarden {
            verify_key: VerifyKey::new(client_public_key)?,
        })
    }

    pub async fn accept(&self, req: &impl ServerRequest) -> Result<ServerResponse> {
        let response = self.verify_key.accept(req).await?;
        let interaction: RawInteraction = req.body_json();
        let response = response.then(match interaction.transform() {
            Some(Interaction::Ping) => ServerResponseBuilder::new()
                .status(StatusCode::Ok)
                .body_json(&InteractionResponse::pong())?,
            Some(Interaction::ApplicationCommand(command)) => todo!(),
            None => Ok(ServerResponseBuilder::new()
                .status(StatusCode::BadRequest)
                .body_str("Server failed to decode the interaction")),
        });
        Ok(response)
    }
}
