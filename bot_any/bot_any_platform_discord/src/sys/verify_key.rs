use ed25519_dalek::{PublicKey, Signature, SignatureError, Verifier};
use hex::FromHexError;
use reqores::{ServerRequest, ServerResponse, ServerResponseBuilder, StatusCode};
use thiserror::Error;

#[derive(Debug, Error)]
pub enum VerifyKeyError {
    #[error("Cannot convert string into hex: {0}")]
    FromHex(#[from] FromHexError),
    #[error("Failed to decode signature: {0}")]
    Signature(#[from] SignatureError),
}

pub struct VerifyKey {
    pub client_public_key: PublicKey,
}

impl VerifyKey {
    pub fn new(client_public_key: &str) -> Result<Self, VerifyKeyError> {
        Ok(VerifyKey {
            client_public_key: PublicKey::from_bytes(&hex::decode(client_public_key)?)?,
        })
    }
}

/**
 * Creates a middleware function for use in Express-compatible web servers.
 *
 * @param clientPublicKey - The public key from the Discord developer dashboard
 * @returns The middleware function
 */
impl VerifyKey {
    fn accept(&self, req: impl ServerRequest) -> Result<ServerResponse, VerifyKeyError> {
        let timestamp = req.header("X-Signature-Timestamp").unwrap_or_default();
        let timestamp = timestamp.as_bytes();

        let body = req.body();

        let message = &[timestamp, body].concat();

        let signature = req.header("X-Signature-Ed25519").unwrap_or_default();
        let signature = Signature::from_bytes(&hex::decode(signature)?)?;

        if self.client_public_key.verify(message, &signature).is_err() {
            Ok(ServerResponseBuilder::new()
                .status(StatusCode::Forbidden)
                .body(
                    "[discord-interactions] Invalid signature"
                        .as_bytes()
                        .to_vec(),
                ))
        } else {
            Ok(ServerResponseBuilder::new().build())
        }
    }
}
