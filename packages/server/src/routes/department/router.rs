use app::{services::department::DepartmentService, utils::query::PaginatedQuery};
use axum::{
    Extension, Json, Router,
    extract::Query,
    routing::{delete, get, patch, post},
};
use sea_orm::DatabaseConnection;
use utoipa::OpenApi;

use crate::{
    dto::PaginatedQueryDto,
    response::{ApiResponse, Null, PaginatedData, ResponseJson},
    result::ServerResult,
    routes::department::dto::{
        CreateDepartmentResponseDto, DeleteDepartmentsRequestDto, UpdateDepartmentRequestDto,
    },
};

use super::dto::{CreateDepartmentRequestDto, DepartmentDto, FilterDepartmentsDto};

#[derive(OpenApi)]
#[openapi(paths(
    create_department,
    query_departments_by_page,
    delete_departments,
    update_department
))]
pub(crate) struct ApiDoc;

pub(crate) fn init() -> Router {
    Router::new()
        .route("/createDepartment", post(create_department))
        .route("/queryDepartmentsByPage", get(query_departments_by_page))
        .route("/deleteDepartments", delete(delete_departments))
        .route("/updateDepartment", patch(update_department))
}

/// Query departments by page
#[utoipa::path(
    operation_id = "queryDepartmentsByPage",
    description = "Query departments by page",
    get,
    path = "/queryDepartmentsByPage",
    params(PaginatedQueryDto, FilterDepartmentsDto),
    responses(
        (status = OK, description = "ok", body = ResponseJson<PaginatedData<DepartmentDto>>)
    )
)]
pub async fn query_departments_by_page(
    Extension(conn): Extension<DatabaseConnection>,
    Query(query): Query<PaginatedQuery<FilterDepartmentsDto>>,
) -> ServerResult<ApiResponse> {
    let department_service = DepartmentService::new(conn);

    let (records, total) = department_service.query_departments_by_page(query).await?;
    let records = records
        .into_iter()
        .map(DepartmentDto::from)
        .collect::<Vec<_>>();

    Ok(ApiResponse::json(PaginatedData { records, total }))
}

/// Create department
#[utoipa::path(
    operation_id = "createDepartment",
    description = "Create department",
    post,
    path = "/createDepartment",
    request_body = CreateDepartmentRequestDto,
    responses(
        (status = OK, description = "ok", body = ResponseJson<CreateDepartmentResponseDto>)
    )
)]
pub async fn create_department(
    Extension(conn): Extension<DatabaseConnection>,
    Json(params): Json<CreateDepartmentRequestDto>,
) -> ServerResult<ApiResponse> {
    let department_service = DepartmentService::new(conn);

    let group_id = department_service.create_department(params.into()).await?;

    Ok(ApiResponse::json(CreateDepartmentResponseDto(group_id)))
}

/// Delete departments
#[utoipa::path(
    operation_id = "deleteDepartments",
    description = "Delete departments",
    delete,
    path = "/deleteDepartments",
    request_body = DeleteDepartmentsRequestDto,
    responses(
        (status = OK, description = "ok", body = ResponseJson<Null>)
    )
)]
pub async fn delete_departments(
    Extension(conn): Extension<DatabaseConnection>,
    Json(params): Json<DeleteDepartmentsRequestDto>,
) -> ServerResult<ApiResponse> {
    let department_service = DepartmentService::new(conn);
    department_service.delete_departments(params.into()).await?;
    Ok(ApiResponse::null())
}

/// Update department
#[utoipa::path(
    operation_id = "updateDepartment",
    description = "Update department",
    patch,
    path = "/updateDepartment",
    request_body = UpdateDepartmentRequestDto,
    responses(
        (status = OK, description = "ok", body = ResponseJson<Null>)
    )
)]
pub async fn update_department(
    Extension(conn): Extension<DatabaseConnection>,
    Json(params): Json<UpdateDepartmentRequestDto>,
) -> ServerResult<ApiResponse> {
    let department_service = DepartmentService::new(conn);
    department_service.update_department(params.into()).await?;
    Ok(ApiResponse::null())
}
