use axum::{Router, routing::get};
use http::StatusCode;
use tower_cookies::{Cookie, Cookies};
use utoipa::OpenApi;

use crate::{
    error::Exception,
    rest::{RestResponse, RestResponseJson},
    result::ServerResult,
    state::AppState,
};

use super::dto::SessionDto;

#[derive(OpenApi)]
#[openapi(paths(query_session))]
pub(crate) struct ApiDoc;

pub(crate) fn init() -> Router<AppState> {
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
pub async fn query_session(cookies: Cookies) -> ServerResult<RestResponseJson<SessionDto>> {
    let visited = cookies
        .get("visited")
        .and_then(|c| c.value().parse().ok())
        .unwrap_or(0);

    if visited > 10 {
        cookies.remove(Cookie::new("visited", ""));
        Err(Exception::new("unknown").status(StatusCode::OK).into())
        // Ok(RestResponse::json("Counter has been reset".to_string()))
    } else {
        cookies.add(Cookie::new("visited", (visited + 1).to_string()));
        Ok(RestResponse::json(SessionDto {
            name: format!("You've been here {} times before", visited),
        }))
    }
}
