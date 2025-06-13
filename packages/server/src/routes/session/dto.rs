use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct SessionInfoDto {
    pub account: String,
    pub nickname: Option<String>,
    pub permissions: Vec<String>,
}

#[derive(Serialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub struct SessionDto {
    pub id: Uuid,
    pub platform: Option<String>,
    pub agent: Option<String>,
    pub created_at: DateTime<Utc>,
}

#[derive(Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query, rename_all = "camelCase")]
pub struct DeleteSessionDto {
    pub id: Uuid,
}
