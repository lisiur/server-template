use app::{
    models::user::User,
    services::user::{delete_user::DeleteUsersParams, query_user::FilterUsersParams},
};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use shared::enums::Gender;
use utoipa::ToSchema;
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

#[derive(Debug, Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct UserFilterDto {
    pub account: Option<String>,
}

impl From<UserFilterDto> for FilterUsersParams {
    fn from(value: UserFilterDto) -> Self {
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
        DeleteUsersParams(value.0)
    }
}
