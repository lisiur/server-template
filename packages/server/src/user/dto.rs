use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Debug, ToSchema, Deserialize)]
pub struct CreateUserDto {
    pub account: String,
    pub password: String,
}