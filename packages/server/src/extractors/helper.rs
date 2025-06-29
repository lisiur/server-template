use axum::extract::FromRequestParts;
use http::request::Parts;
use rsa::{RsaPrivateKey, RsaPublicKey};
use sea_orm::DatabaseConnection;
use shared::utils;

use crate::result::ServerResult;

#[derive(Debug)]
pub struct Helper {
    #[allow(dead_code)]
    conn: DatabaseConnection,
    private_key: RsaPrivateKey,
    public_key: RsaPublicKey,
}

impl<S> FromRequestParts<S> for Helper
where
    S: Send + Sync,
{
    type Rejection = (http::StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let conn = parts.extensions.get::<DatabaseConnection>().unwrap();
        let private_key = parts.extensions.get::<RsaPrivateKey>().unwrap();
        let public_key = parts.extensions.get::<RsaPublicKey>().unwrap();

        Ok(Helper {
            conn: conn.clone(),
            private_key: private_key.clone(),
            public_key: public_key.clone(),
        })
    }
}

impl Helper {
    #[allow(dead_code)]
    pub fn sign_rsa(&self, plaintext: &str) -> ServerResult<String> {
        let ciphertext = utils::sign_rsa(&self.private_key, plaintext.as_bytes())?;
        let ciphertext_base64 = utils::encode_base64(&ciphertext);
        Ok(ciphertext_base64)
    }

    #[allow(dead_code)]
    pub fn verify_rsa(&self, msg: &str, signature: &str) -> ServerResult<()> {
        let signature = utils::decode_base64(signature)?;
        utils::verify_rsa(&self.public_key, msg, &signature)?;
        Ok(())
    }

    pub fn decrypt_rsa(&self, ciphertext_base64: &str) -> ServerResult<String> {
        let ciphertext = utils::decode_base64(ciphertext_base64)?;
        let plaintext = utils::decrypt_rsa_to_utf8(&self.private_key, &ciphertext)?;

        Ok(plaintext)
    }

    #[allow(dead_code)]
    pub fn encrypt_rsa(&self, plaintext: &str) -> ServerResult<String> {
        let ciphertext = utils::encrypt_rsa(&self.public_key, plaintext.as_bytes())?;
        let ciphertext_base64 = utils::encode_base64(&ciphertext);

        Ok(ciphertext_base64)
    }
}
