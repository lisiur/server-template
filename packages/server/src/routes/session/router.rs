use app::services::{auth::AuthService, auth_token::AuthTokenService, user::UserService};
use axum::{Extension, extract::Query, response::IntoResponse};
use sea_orm::DatabaseConnection;
use utoipa::OpenApi;

use crate::{
    error::ServerExceptionCode,
    extractors::{app_service::AppService, auth_session::AuthSession},
    init_router,
    response::{ApiResponse, Null, ResponseJson},
    result::ServerResult,
    routes::session::dto::{DeleteSessionDto, SessionDto},
};

use super::dto::SessionInfoDto;

#[derive(OpenApi)]
#[openapi(paths(
    query_session,
    query_session_permissions,
    query_active_sessions,
    delete_session
))]
pub(crate) struct ApiDoc;
init_router!(
    query_session,
    query_session_permissions,
    query_active_sessions,
    delete_session
);

#[utoipa::path(
    get,
    path = "/querySession",
    responses(
        (status = OK, description = "ok", body = ResponseJson<SessionInfoDto>)
    )
)]
/// Query session
pub async fn query_session(
    Extension(conn): Extension<DatabaseConnection>,
    auth_session: AuthSession,
) -> ServerResult<ApiResponse> {
    let user_service = UserService::new(conn);
    let user_id = auth_session.payload.user_id;
    let user = user_service.query_user_by_id(user_id).await?;
    Ok(ApiResponse::json(SessionInfoDto {
        account: user.account,
        nickname: user.nickname,
        permissions: auth_session.payload.permissions,
    }))
}

#[utoipa::path(
    get,
    path = "/querySessionPermissions",
    responses(
        (status = OK, description = "ok", body = ResponseJson<Null>)
    )
)]
/// Query session Permissions
pub async fn query_session_permissions(
    auth_session: AuthSession,
    auth_service: AppService<AuthService>,
) -> ServerResult<ApiResponse> {
    let user_id = auth_session.payload.user_id;
    let (user_permissions, _) = auth_service.query_user_permissions(user_id).await?;
    Ok(ApiResponse::json(user_permissions))
}

#[utoipa::path(
    get,
    path = "/queryActiveSessions",
    responses(
        (status = OK, description = "ok", body = ResponseJson<Vec<SessionDto>>)
    )
)]
/// Query active sessions
pub async fn query_active_sessions(
    Extension(conn): Extension<DatabaseConnection>,
    auth_session: AuthSession,
) -> ServerResult<ApiResponse> {
    let user_id = auth_session.payload.user_id;
    let auth_token_service = AuthTokenService::new(conn);
    let tokens = auth_token_service
        .query_auth_tokens_by_ref_id(user_id)
        .await?;
    Ok(ApiResponse::json(
        tokens
            .into_iter()
            .map(|token| SessionDto {
                id: token.id,
                platform: token.platform,
                agent: token.agent,
                created_at: token.created_at,
            })
            .collect::<Vec<_>>(),
    ))
}

#[utoipa::path(
    delete,
    path = "/deleteSession",
    params(DeleteSessionDto),
    responses(
        (status = OK, description = "ok", body = ResponseJson<Null>)
    )
)]
/// Delete session
pub async fn delete_session(
    auth_session: AuthSession,
    Extension(conn): Extension<DatabaseConnection>,
    Query(query): Query<DeleteSessionDto>,
) -> ServerResult<impl IntoResponse> {
    let auth_token_service = AuthTokenService::new(conn);

    let Some(target) = auth_token_service.query_auth_token_by_id(query.id).await? else {
        return Ok(ApiResponse::null());
    };

    let valid_user_id = auth_session.payload.user_id;

    if valid_user_id == target.ref_id {
        auth_token_service.delete_auth_token_by_id(query.id).await?;
    } else {
        return Err(ServerExceptionCode::Forbidden.into());
    }

    Ok(ApiResponse::null())
}
