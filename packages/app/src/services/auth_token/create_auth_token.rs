use chrono::{DateTime, Duration, Utc};
use entity::auth_tokens;
use jsonwebtoken::{EncodingKey, Header, encode};
use sea_orm::{
    ActiveValue::{NotSet, Set},
    EntityTrait,
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    models::auth_token::{AuthTokenKind, SessionTokenClaims, SessionTokenPayload},
    result::AppResult,
    services::auth_token::AuthTokenService,
};

const SECRET: &[u8; 6] = b"secret";

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSessionTokenParams {
    pub ip: Option<String>,
    pub platform: Option<String>,
    pub agent: Option<String>,
    pub expired_at: Option<DateTime<Utc>>,
    pub user_id: Uuid,
    pub permissions: Vec<String>,
    pub roles: Vec<Uuid>,
    pub groups: Vec<Uuid>,
    pub departments: Vec<Uuid>,
}

impl AuthTokenService {
    pub async fn create_session_token(&self, params: CreateSessionTokenParams) -> AppResult<Uuid> {
        let claims = SessionTokenClaims {
            sub: params.user_id,
            exp: params
                .expired_at
                .unwrap_or_else(|| Utc::now() + Duration::days(365))
                .timestamp(),
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(SECRET),
        )
        .unwrap();

        let payload = SessionTokenPayload {
            user_id: params.user_id,
            permissions: params.permissions,
            roles: params.roles,
            groups: params.groups,
            departments: params.departments,
        };

        let active_model = auth_tokens::ActiveModel {
            id: Set(Uuid::new_v4()),
            kind: Set(AuthTokenKind::Session.to_string()),
            ref_id: Set(params.user_id),
            token: Set(token),
            ip: Set(params.ip),
            platform: Set(params.platform),
            agent: Set(params.agent),
            payload: Set(serde_json::to_string(&payload).unwrap()),
            expired_at: Set(params.expired_at.map(Into::into)),
            is_deleted: NotSet,
            created_at: NotSet,
            updated_at: NotSet,
        };

        let result = auth_tokens::Entity::insert(active_model)
            .exec(&self.conn)
            .await?;

        Ok(result.last_insert_id)
    }
}
