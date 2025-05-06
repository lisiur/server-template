use axum::response::{IntoResponse, Response};
use serde::Serialize;
use utoipa::ToSchema;

use crate::result::ServerResult;

#[derive(Serialize, ToSchema)]
pub struct RestResponseJson<T: Serialize> {
    data: Option<T>,
}

impl<T: Serialize> IntoResponse for RestResponseJson<T> {
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}

impl<T: Serialize> From<RestResponseJson<T>> for ServerResult<RestResponseJson<T>> {
    fn from(value: RestResponseJson<T>) -> Self {
        Ok(value)
    }
}

#[derive(Serialize)]
pub struct RestResponseErrorJson {
    code: String,
    message: String,
}

impl RestResponseErrorJson {
    pub fn new(code: String, message: String) -> Self {
        Self { code, message }
    }
}

impl IntoResponse for RestResponseErrorJson {
    fn into_response(self) -> Response {
        axum::Json(self).into_response()
    }
}

pub struct RestResponse;

impl RestResponse {
    pub fn json<T: Serialize>(data: T) -> RestResponseJson<T> {
        RestResponseJson { data: Some(data) }
    }

    #[allow(dead_code)]
    pub fn null() -> RestResponseJson<()> {
        RestResponseJson { data: None }
    }
}
