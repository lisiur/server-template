use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
pub struct SessionDto {
    /// name
    pub name: String,
}
