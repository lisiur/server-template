use app::services::auth::AuthService;
use axum::{
    Extension, Json, Router,
    extract::Query,
    routing::{get, post},
};
use axum_extra::{TypedHeader, extract::cookie::Cookie, headers::UserAgent};
use sea_orm::DatabaseConnection;
use utoipa::OpenApi;

use crate::{
    extractors::auth_session::{AuthSession, SESSION_ID_KEY},
    response::{ApiResponse, Null, ResponseJson},
    result::ServerResult,
    routes::{
        auth::dto::{
            LoginRequestDto, LoginResponseDto, QueryDepartmentPermissionsDto,
            QueryGroupPermissionsDto, QueryUserPermissionsDto,
        },
        permission::dto::PermissionDto,
    },
};

use super::dto::{AssignUserPermissionsDto, GroupTreePermissionsDto};

#[derive(OpenApi)]
#[openapi(paths(
    login,
    logout,
    assign_user_permissions,
    query_user_permissions,
    query_group_permissions,
    query_department_permissions,
))]
pub(crate) struct ApiDoc;

/// Assign user permissions
pub(crate) fn init() -> Router {
    Router::new()
        .route("/login", post(login))
        .route("/logout", get(logout))
        .route("/assignUserPermissions", post(assign_user_permissions))
        .route("/queryUserPermissions", get(query_user_permissions))
        .route("/queryGroupPermissions", get(query_group_permissions))
        .route(
            "/queryDepartmentPermissions",
            get(query_department_permissions),
        )
}

/// Login
#[utoipa::path(
    operation_id = "login",
    description = "Login",
    post,
    path = "/login",
    request_body = LoginRequestDto,
    responses(
        (status = OK, description = "ok", body = ResponseJson<LoginResponseDto>)
    )
)]
pub async fn login(
    Extension(conn): Extension<DatabaseConnection>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    Json(mut params): Json<LoginRequestDto>,
) -> ServerResult<ApiResponse> {
    params.agent = Some(user_agent.to_string());

    let auth_service = AuthService::new(conn);
    let (auth_token_id, user) = auth_service.login(params.into()).await?;

    let mut cookie = Cookie::new(SESSION_ID_KEY, auth_token_id.to_string());
    cookie.set_path("/");
    cookie.set_http_only(true);

    let response_dto = LoginResponseDto {
        user_id: user.id,
        account: user.account,
    };

    let mut response = ApiResponse::default();
    response.set_cookie(cookie);
    response.set_body_json(response_dto);

    Ok(response)
}

/// Logout
#[utoipa::path(
    operation_id = "logout",
    description = "Logout",
    get,
    path = "/logout",
    responses(
        (status = OK, description = "ok", body = ResponseJson<Null>)
    )
)]
pub async fn logout(auth_session: AuthSession) -> ServerResult<ApiResponse> {
    auth_session.logout().await?;
    Ok(ApiResponse::null())
}

/// Assign user permissions
#[utoipa::path(
    operation_id = "assignUserPermissions",
    description = "Assign user permissions",
    post,
    path = "/assignUserPermissions",
    request_body = AssignUserPermissionsDto,
    responses(
        (status = OK, description = "ok", body = ResponseJson<Null>)
    )
)]
pub async fn assign_user_permissions(
    Extension(conn): Extension<DatabaseConnection>,
    Json(params): Json<AssignUserPermissionsDto>,
) -> ServerResult<ApiResponse> {
    let auth_service = AuthService::new(conn);

    auth_service.assign_user_permissions(params.into()).await?;

    Ok(ApiResponse::null())
}

/// Query user permission
#[utoipa::path(
    operation_id = "queryUserPermissions",
    description = "Query user permissions",
    get,
    path = "/queryUserPermissions",
    params(QueryUserPermissionsDto),
    responses(
        (status = OK, description = "ok", body = ResponseJson<Vec<PermissionDto>>)
    )
)]
pub async fn query_user_permissions(
    Extension(conn): Extension<DatabaseConnection>,
    Query(query): Query<QueryUserPermissionsDto>,
) -> ServerResult<ApiResponse> {
    let auth_service = AuthService::new(conn);

    let res = auth_service.query_user_permissions(query.user_id).await?;
    let res = res.into_iter().map(PermissionDto::from).collect::<Vec<_>>();

    Ok(ApiResponse::json(res))
}

/// Query group permissions
#[utoipa::path(
    operation_id = "queryGroupPermissions",
    description = "Query group permissions",
    get,
    path = "/queryGroupPermissions",
    params(QueryGroupPermissionsDto),
    responses(
        (status = OK, description = "ok", body = ResponseJson<GroupTreePermissionsDto>)
    )
)]
pub async fn query_group_permissions(
    Extension(conn): Extension<DatabaseConnection>,
    Query(query): Query<QueryGroupPermissionsDto>,
) -> ServerResult<ApiResponse> {
    let auth_service = AuthService::new(conn);

    let res = auth_service.query_group_permissions(query.group_id).await?;

    Ok(ApiResponse::json(
        res.into_iter()
            .map(PermissionDto::from)
            .collect::<Vec<PermissionDto>>(),
    ))
}

/// Query department permissions
#[utoipa::path(
    operation_id = "queryDepartmentPermissions",
    description = "Query department permissions",
    get,
    path = "/queryDepartmentPermissions",
    params(QueryDepartmentPermissionsDto),
    responses(
        (status = OK, description = "ok", body = ResponseJson<Vec<PermissionDto>>)
    )
)]
pub async fn query_department_permissions(
    Extension(conn): Extension<DatabaseConnection>,
    Query(query): Query<QueryDepartmentPermissionsDto>,
) -> ServerResult<ApiResponse> {
    let auth_service = AuthService::new(conn);

    let res = auth_service
        .query_department_permissions(query.department_id)
        .await?;

    Ok(ApiResponse::json(
        res.into_iter()
            .map(PermissionDto::from)
            .collect::<Vec<PermissionDto>>(),
    ))
}
