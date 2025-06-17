use uuid::Uuid;

use crate::{
    result::AppResult,
    services::{auth::AuthService, auth_token::AuthTokenService},
};

impl AuthService {
    pub async fn logout(&self, session_id: Uuid) -> AppResult<()> {
        let auth_token_service = AuthTokenService::new(self.0.clone());
        auth_token_service
            .delete_auth_token_by_id(session_id)
            .await?;

        Ok(())
    }

    pub async fn logout_all(&self, user_id: Uuid) -> AppResult<()> {
        let auth_token_service = AuthTokenService::new(self.0.clone());
        auth_token_service
            .delete_auth_token_by_ref_id(user_id)
            .await?;

        Ok(())
    }
}
