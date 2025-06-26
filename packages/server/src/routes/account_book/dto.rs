use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, IntoParams)]
pub struct FilterAccountBooksDto {}

#[derive(Serialize, ToSchema)]
pub struct AccountBookDto {}

#[derive(Deserialize, ToSchema)]
pub struct CreateAccountBookDto {}

#[derive(Serialize, ToSchema)]
pub struct CreateAccountBookResponseDto {}
