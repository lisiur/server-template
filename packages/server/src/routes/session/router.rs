use axum::{Router, routing::get};
use http::StatusCode;
use utoipa::OpenApi;

use crate::{
    error::ServerException,
    rest::{RestResponse, RestResponseJson},
    result::ServerResult,
};

use super::dto::SessionDto;

#[derive(OpenApi)]
#[openapi(paths(query_session))]
pub(crate) struct ApiDoc;

pub(crate) fn init() -> Router {
    Router::new().route("/querySession", get(query_session))
}

#[utoipa::path(
    get,
    path = "/querySession",
    responses(
        (status = OK, description = "ok", body = RestResponseJson<SessionDto>)
    )
)]
/// Query session
pub async fn query_session() -> ServerResult<RestResponseJson<SessionDto>> {
    todo!()
}
