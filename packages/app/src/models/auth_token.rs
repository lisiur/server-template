use std::str::FromStr;

use chrono::{DateTime, Utc};
use entity::auth_tokens;
use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use uuid::Uuid;

pub struct AuthToken {
    pub id: Uuid,
    pub kind: AuthTokenKind,
    pub ref_id: Uuid,
    pub token: String,
    pub ip: Option<String>,
    pub platform: Option<String>,
    pub agent: Option<String>,
    pub payload: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<auth_tokens::Model> for AuthToken {
    fn from(model: auth_tokens::Model) -> Self {
        Self {
            id: model.id,
            kind: AuthTokenKind::from_str(&model.kind).unwrap(),
            ref_id: model.ref_id,
            token: model.token,
            agent: model.agent,
            ip: model.ip,
            platform: model.platform,
            payload: model.payload,
            created_at: model.created_at.into(),
            updated_at: model.updated_at.into(),
        }
    }
}

impl AuthToken {}

#[derive(Debug, Serialize, Deserialize, Display, EnumString)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "snake_case")]
pub enum AuthTokenKind {
    Session,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionTokenClaims {
    pub sub: Uuid,
    pub exp: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SessionTokenPayload {
    pub user_id: Uuid,
    pub permissions: Vec<String>,
    pub roles: Vec<Uuid>,
    pub groups: Vec<Uuid>,
    pub departments: Vec<Uuid>,
}
