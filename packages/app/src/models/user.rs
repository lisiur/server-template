use chrono::{DateTime, Utc};
use entity::users;
use serde::{Deserialize, Serialize};
use strum_macros::{Display, EnumString};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct User {
    pub id: Uuid,
    pub account: String,
    pub nickname: Option<String>,
    pub real_name: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub email_verified: bool,
    pub avatar_url: Option<String>,
    pub gender: Gender,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<users::Model> for User {
    fn from(value: users::Model) -> Self {
        Self {
            id: value.id,
            account: value.account,
            nickname: value.nickname,
            real_name: value.real_name,
            phone: value.phone,
            email: value.email,
            email_verified: value.email_verified,
            avatar_url: value.avatar_url,
            gender: value.gender.as_str().try_into().unwrap(),
            created_at: value.created_at.into(),
            updated_at: value.updated_at.into(),
        }
    }
}

#[derive(
    Clone, Debug, PartialEq, Eq, Serialize, Deserialize, Display, EnumString, Default, ToSchema,
)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "camelCase")]
pub enum Gender {
    #[default]
    Unknown,
    Male,
    Female,
}
