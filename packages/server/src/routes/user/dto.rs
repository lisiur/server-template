use app::{
    models::user::{Gender, User},
    services::user::{delete_users::DeleteUsersParams, query_users::FilterUsersParams},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(Debug, ToSchema, Deserialize)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct CreateUserDto {
    pub account: String,
    pub password: String,
}

#[derive(Debug, ToSchema, Serialize)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct UserDto {
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

impl From<User> for UserDto {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            account: value.account,
            nickname: value.nickname,
            real_name: value.real_name,
            phone: value.phone,
            email: value.email,
            email_verified: value.email_verified,
            avatar_url: value.avatar_url,
            gender: value.gender,
            created_at: value.created_at.into(),
            updated_at: value.updated_at.into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query, rename_all = "camelCase")]
pub struct FilterUserDto {
    pub account: Option<String>,
}

impl From<FilterUserDto> for FilterUsersParams {
    fn from(value: FilterUserDto) -> Self {
        FilterUsersParams {
            account: value.account,
        }
    }
}

/// User id list
#[derive(Debug, ToSchema, Deserialize)]
pub struct DeleteUsersRequestDto(Vec<Uuid>);

impl From<DeleteUsersRequestDto> for DeleteUsersParams {
    fn from(value: DeleteUsersRequestDto) -> Self {
        Self(value.0)
    }
}
