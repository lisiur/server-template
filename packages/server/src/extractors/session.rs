use std::str::FromStr;

use app::{App, models::auth_token::SessionTokenPayload, services::auth_token::AuthTokenService};
use axum::extract::FromRequestParts;
use axum_extra::extract::CookieJar;
use http::request::Parts;
use uuid::Uuid;

use crate::{error::ServerExceptionCode, result::ServerResult};

pub const SESSION_ID_KEY: &str = "id";

#[derive(Debug)]
pub struct Session {
    pub session_id: Uuid,
    pub payload: SessionTokenPayload,
}

impl Session {
    #[allow(unused)]
    pub fn has_permission(&self, permission_code: impl ToString) -> bool {
        let permission_code = permission_code.to_string();
        self.payload.permissions.contains(&permission_code)
    }

    #[allow(unused)]
    pub fn assert_has_permission(&self, permission_code: impl ToString) -> ServerResult<()> {
        if !self.has_permission(permission_code) {
            return Err(ServerExceptionCode::Forbidden.into());
        }

        Ok(())
    }

    #[allow(unused)]
    pub fn has_any_permissions(
        &self,
        permission_codes: impl IntoIterator<Item = impl ToString>,
    ) -> bool {
        for permission_code in permission_codes {
            if self.has_permission(permission_code) {
                return true;
            }
        }
        false
    }

    #[allow(unused)]
    pub fn assert_has_any_permissions(
        &self,
        permission_codes: impl IntoIterator<Item = impl ToString>,
    ) -> ServerResult<()> {
        if !self.has_any_permissions(permission_codes) {
            return Err(ServerExceptionCode::Forbidden.into());
        }
        Ok(())
    }

    #[allow(unused)]
    pub fn has_all_permissions(
        &self,
        permission_codes: impl IntoIterator<Item = impl ToString>,
    ) -> bool {
        for permission_code in permission_codes {
            if !self.has_permission(permission_code) {
                return false;
            }
        }
        true
    }

    #[allow(unused)]
    pub fn assert_has_all_permissions(
        &self,
        permission_codes: impl IntoIterator<Item = impl ToString>,
    ) -> ServerResult<()> {
        if !self.has_all_permissions(permission_codes) {
            return Err(ServerExceptionCode::Forbidden.into());
        }
        Ok(())
    }
}

impl<S> FromRequestParts<S> for Session
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

        let app = parts.extensions.get::<App>().unwrap();
        let auth_token_service = AuthTokenService::new(app.clone());

        let Ok(Some(auth_token)) = auth_token_service.query_auth_token_by_id(session_id).await
        else {
            return Err((http::StatusCode::UNAUTHORIZED, "Unauthorized"));
        };

        let payload = serde_json::from_str::<SessionTokenPayload>(&auth_token.payload).unwrap();

        Ok(Session {
            session_id,
            payload,
        })
    }
}
