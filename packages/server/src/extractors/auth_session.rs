use axum::{Extension, extract::FromRequestParts};
use axum_extra::extract::CookieJar;
use http::request::Parts;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use tower_sessions::Session;
const AUTH_SESSION_PREFIX_KEY: &str = "auth_session_";

#[derive(Debug)]
pub struct AuthSession {
    conn: DatabaseConnection,
    session_id: String,
    session: Session,
}

impl AuthSession {
    pub async fn login_with_account_and_password(&mut self, account: &str, password: &str) {}
}

impl<S> FromRequestParts<S> for AuthSession
where
    S: Send + Sync,
{
    type Rejection = (http::StatusCode, &'static str);

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let jar = CookieJar::from_request_parts(parts, state).await.unwrap();
        let Some(cookie) = jar.get("session_id") else {
            return Err((http::StatusCode::UNAUTHORIZED, "Unauthorized"));
        };

        let session_id = cookie.value().to_string();

        let session = Session::from_request_parts(parts, state).await?;

        let Extension(conn) = Extension::<DatabaseConnection>::from_request_parts(parts, state)
            .await
            .unwrap();

        Ok(AuthSession {
            conn,
            session_id,
            session,
        })

        // let Ok(Some(auth_session)) = session.get::<AuthSession>(&session_id).await else {
        //     return Err((http::StatusCode::UNAUTHORIZED, "Unauthorized"));
        // };

        // Ok(auth_session)
    }
}
