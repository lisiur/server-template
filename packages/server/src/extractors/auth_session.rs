use std::str::FromStr;

use app::{models::auth_token::SessionTokenPayload, services::auth_token::AuthTokenService};
use axum::extract::FromRequestParts;
use axum_extra::extract::CookieJar;
use http::request::Parts;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::result::ServerResult;
pub const SESSION_ID_KEY: &str = "id";

#[derive(Debug)]
pub struct AuthSession {
    pub(self) conn: DatabaseConnection,
    pub session_id: Uuid,
    pub payload: SessionTokenPayload,
}

impl AuthSession {
    pub async fn logout(&self) -> ServerResult<()> {
        let auth_token_service = AuthTokenService::new(self.conn.clone());
        auth_token_service
            .delete_auth_token_by_id(self.session_id)
            .await?;
        Ok(())
    }
}

impl<S> FromRequestParts<S> for AuthSession
where
    S: Send + Sync,
{
    type Rejection = (http::StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_request_parts(parts, state).await.unwrap();
        let Some(cookie) = jar.get(SESSION_ID_KEY) else {
            return Err((http::StatusCode::UNAUTHORIZED, "Unauthorized"));
        };

        let Ok(session_id) = Uuid::from_str(cookie.value()) else {
            return Err((http::StatusCode::UNAUTHORIZED, "Unauthorized"));
        };

        let conn = parts.extensions.get::<DatabaseConnection>().unwrap();
        let auth_token_service = AuthTokenService::new(conn.clone());

        let Ok(Some(auth_token)) = auth_token_service.query_auth_token_by_id(session_id).await
        else {
            return Err((http::StatusCode::UNAUTHORIZED, "Unauthorized"));
        };

        let payload = serde_json::from_str::<SessionTokenPayload>(&auth_token.payload).unwrap();

        Ok(AuthSession {
            conn: conn.clone(),
            session_id,
            payload,
        })
    }
}
