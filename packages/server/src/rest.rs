use axum::response::{IntoResponse, Response};
use serde::Serialize;
use utoipa::ToSchema;

use crate::result::ServerResult;

#[derive(ToSchema)]
pub struct Null;

impl Serialize for Null {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_none() // Serializes as `null`
    }
}

#[derive(Serialize, ToSchema)]
pub struct RestResponseJson<T: Serialize> {
    data: T,
}

pub type RestResponseJsonNull = RestResponseJson<Null>;

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
        RestResponseJson { data }
    }

    #[allow(dead_code)]
    pub fn null() -> RestResponseJson<Null> {
        RestResponseJson { data: Null }
    }
}

#[derive(Serialize, ToSchema)]
pub struct PaginatedData<T> {
    pub records: Vec<T>,
    pub total: i64,
}

impl<T, U: From<T>> From<(Vec<T>, i64)> for PaginatedData<U> {
    fn from(value: (Vec<T>, i64)) -> Self {
        Self {
            records: value.0.into_iter().map(|item| item.into()).collect(),
            total: value.1,
        }
    }
}
