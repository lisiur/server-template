use app::services::auth::AuthService;
use axum::{Extension, Json, extract::Query};
use axum_extra::{TypedHeader, extract::cookie::Cookie, headers::UserAgent};
use sea_orm::DatabaseConnection;
use shared::enums::OperationPermission;
use time::Duration;
use utoipa::OpenApi;

use crate::{
    extractors::{
        app_service::AppService,
        helper::Helper,
        session::{SESSION_ID_KEY, Session},
    },
    init_router,
    response::{ApiResponse, Null, ResponseJson},
    result::ServerResult,
    routes::auth::dto::{
        LoginRequestDto, LoginResponseDto, QueryDepartmentPermissionsDto, QueryGroupPermissionsDto,
        QueryPermissionGroupPermissionsDto, QueryUserPermissionsDto, RegisterRequestDto,
    },
};

use super::dto::{AssignUserPermissionsDto, QueryRoleGroupPermissionsDto, QueryRolePermissionsDto};

#[derive(OpenApi)]
#[openapi(paths(
    register,
    login,
    logout,
    logout_all,
    assign_user_permissions,
    query_user_permissions,
    query_user_group_permissions,
    query_department_permissions,
    query_role_permissions,
    query_role_group_permissions,
    query_permission_group_permissions,
))]
pub(crate) struct ApiDoc;
init_router!(
    register,
    login,
    logout,
    logout_all,
    assign_user_permissions,
    query_user_permissions,
    query_user_group_permissions,
    query_department_permissions,
    query_role_permissions,
    query_role_group_permissions,
    query_permission_group_permissions
);

/// Register
#[utoipa::path(
    operation_id = "register",
    post,
    path = "/register",
    request_body = RegisterRequestDto,
    responses(
        (status = OK, description = "ok", body = ResponseJson<LoginResponseDto>)
    )
)]
pub async fn register(
    helper: Helper,
    auth_service: AppService<AuthService>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    Json(mut params): Json<RegisterRequestDto>,
) -> ServerResult<ApiResponse> {
    params.agent = Some(user_agent.to_string());
    params.password = helper.decrypt_rsa(&params.password)?;

    let (auth_token_id, user) = auth_service.register(params.into()).await?;

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

/// Login
#[utoipa::path(
    operation_id = "login",
    post,
    path = "/login",
    request_body = LoginRequestDto,
    responses(
        (status = OK, description = "ok", body = ResponseJson<LoginResponseDto>)
    )
)]
pub async fn login(
    helper: Helper,
    auth_service: AppService<AuthService>,
    TypedHeader(user_agent): TypedHeader<UserAgent>,
    Json(mut params): Json<LoginRequestDto>,
) -> ServerResult<ApiResponse> {
    params.agent = Some(user_agent.to_string());
    params.password = helper.decrypt_rsa(&params.password)?;

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
pub async fn logout(
    Extension(conn): Extension<DatabaseConnection>,
    auth_session: Session,
) -> ServerResult<ApiResponse> {
    let auth_service = AuthService::new(conn);
    auth_service.logout(auth_session.session_id).await?;

    let mut cookie = Cookie::new(SESSION_ID_KEY, "");
    cookie.set_path("/");
    cookie.set_http_only(true);
    cookie.set_max_age(Duration::seconds(0));

    let mut response = ApiResponse::default();
    response.set_cookie(cookie);

    Ok(ApiResponse::null())
}

/// Logout all
#[utoipa::path(
    operation_id = "logoutAll",
    description = "LogoutAll",
    get,
    path = "/logoutAll",
    responses(
        (status = OK, description = "ok", body = ResponseJson<Null>)
    )
)]
pub async fn logout_all(
    auth_session: Session,
    auth_service: AppService<AuthService>,
) -> ServerResult<ApiResponse> {
    auth_service
        .logout_all(auth_session.payload.user_id)
        .await?;

    let mut cookie = Cookie::new(SESSION_ID_KEY, "");
    cookie.set_path("/");
    cookie.set_http_only(true);
    cookie.set_max_age(Duration::seconds(0));

    let mut response = ApiResponse::default();
    response.set_cookie(cookie);

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
    session: Session,
    auth_service: AppService<AuthService>,
    Json(params): Json<AssignUserPermissionsDto>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::AssignUserPermissions)?;

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
        (status = OK, description = "ok", body = ResponseJson<Null>)
    )
)]
pub async fn query_user_permissions(
    session: Session,
    auth_service: AppService<AuthService>,
    Query(query): Query<QueryUserPermissionsDto>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::QueryUserPermissions)?;

    let res = auth_service.query_user_permissions(query.user_id).await?;

    Ok(ApiResponse::json(res))
}

/// Query user group permissions
#[utoipa::path(
    operation_id = "queryUserGroupPermissions",
    description = "Query user group permissions",
    get,
    path = "/queryUserGroupPermissions",
    params(QueryGroupPermissionsDto),
    responses(
        (status = OK, description = "ok", body = ResponseJson<Null>)
    )
)]
pub async fn query_user_group_permissions(
    session: Session,
    auth_service: AppService<AuthService>,
    Query(query): Query<QueryGroupPermissionsDto>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::QueryGroupPermissions)?;

    let res = auth_service
        .query_user_group_permissions(query.user_group_id)
        .await?;

    Ok(ApiResponse::json(res))
}

/// Query department permissions
#[utoipa::path(
    operation_id = "queryDepartmentPermissions",
    description = "Query department permissions",
    get,
    path = "/queryDepartmentPermissions",
    params(QueryDepartmentPermissionsDto),
    responses(
        (status = OK, description = "ok", body = ResponseJson<Vec<Null>>)
    )
)]
pub async fn query_department_permissions(
    session: Session,
    auth_service: AppService<AuthService>,
    Query(query): Query<QueryDepartmentPermissionsDto>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::QueryDepartmentPermissions)?;

    let res = auth_service
        .query_department_permissions(query.department_id)
        .await?;

    Ok(ApiResponse::json(res))
}

/// Query role permissions
#[utoipa::path(
    operation_id = "queryRolePermissions",
    description = "Query role permissions",
    get,
    path = "/queryRolePermissions",
    params(QueryRolePermissionsDto),
    responses(
        (status = OK, description = "ok", body = ResponseJson<Vec<Null>>)
    )
)]
pub async fn query_role_permissions(
    session: Session,
    auth_service: AppService<AuthService>,
    Query(query): Query<QueryRolePermissionsDto>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::QueryRolePermissions)?;

    let res = auth_service.query_role_permissions(query.role_id).await?;

    Ok(ApiResponse::json(res))
}

/// Query role_group permissions
#[utoipa::path(
    operation_id = "queryRoleGroupPermissions",
    description = "Query role group permissions",
    get,
    path = "/queryRoleGroupPermissions",
    params(QueryRoleGroupPermissionsDto),
    responses(
        (status = OK, description = "ok", body = ResponseJson<Vec<Null>>)
    )
)]
pub async fn query_role_group_permissions(
    session: Session,
    auth_service: AppService<AuthService>,
    Query(query): Query<QueryRoleGroupPermissionsDto>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::QueryRolePermissions)?;

    let res = auth_service
        .query_role_group_permissions(query.role_group_id)
        .await?;

    Ok(ApiResponse::json(res))
}

/// Query permission group permissions
#[utoipa::path(
    operation_id = "queryPermissionGroupPermissions",
    description = "Query permission group permissions",
    get,
    path = "/queryPermissionGroupPermissions",
    params(QueryPermissionGroupPermissionsDto),
    responses(
        (status = OK, description = "ok", body = ResponseJson<Vec<Null>>)
    )
)]
pub async fn query_permission_group_permissions(
    session: Session,
    auth_service: AppService<AuthService>,
    Query(query): Query<QueryPermissionGroupPermissionsDto>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::QueryPermissionGroupPermissions)?;

    let res = auth_service
        .query_permission_group_permissions(query.permission_group_id)
        .await?;

    Ok(ApiResponse::json(res))
}
