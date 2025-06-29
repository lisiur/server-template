use entity::auth_tokens;

use crate::impl_service;
pub mod create_auth_token;
pub mod delete_auth_token;
pub mod query_auth_tokens;

impl_service!(AuthTokenService, auth_tokens::Entity);
