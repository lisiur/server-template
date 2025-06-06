use app::{services::department::DepartmentService, utils::query::PaginatedQuery};
use axum::{
    Json, Router,
    extract::{Query, State},
    routing::{delete, get, patch, post},
};
use utoipa::OpenApi;

use crate::{
    department::dto::{
        CreateDepartmentResponseDto, DeleteDepartmentsRequestDto, UpdateDepartmentRequestDto,
    },
    dto::PaginatedQueryDto,
    rest::{Null, PaginatedData, RestResponse, RestResponseJson},
    result::ServerResult,
    state::AppState,
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

pub(crate) fn init() -> Router<AppState> {
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
        (status = OK, description = "ok", body = RestResponseJson<PaginatedData<DepartmentDto>>)
    )
)]
pub async fn query_departments_by_page(
    State(state): State<AppState>,
    Query(query): Query<PaginatedQuery<FilterDepartmentsDto>>,
) -> ServerResult<RestResponseJson<PaginatedData<DepartmentDto>>> {
    let department_service = DepartmentService::new(state.db_conn);

    let (records, total) = department_service.query_departments_by_page(query).await?;
    let records = records
        .into_iter()
        .map(DepartmentDto::from)
        .collect::<Vec<_>>();

    Ok(RestResponse::json(PaginatedData { records, total }))
}

/// Create department
#[utoipa::path(
    operation_id = "createDepartment",
    description = "Create department",
    post,
    path = "/createDepartment",
    request_body = CreateDepartmentRequestDto,
    responses(
        (status = OK, description = "ok", body = RestResponseJson<CreateDepartmentResponseDto>)
    )
)]
pub async fn create_department(
    State(state): State<AppState>,
    Json(params): Json<CreateDepartmentRequestDto>,
) -> ServerResult<RestResponseJson<CreateDepartmentResponseDto>> {
    let department_service = DepartmentService::new(state.db_conn);

    let group_id = department_service.create_department(params.into()).await?;

    Ok(RestResponse::json(CreateDepartmentResponseDto(group_id)))
}

/// Delete departments
#[utoipa::path(
    operation_id = "deleteDepartments",
    description = "Delete departments",
    delete,
    path = "/deleteDepartments",
    request_body = DeleteDepartmentsRequestDto,
    responses(
        (status = OK, description = "ok", body = RestResponseJson<Null>)
    )
)]
pub async fn delete_departments(
    State(state): State<AppState>,
    Json(params): Json<DeleteDepartmentsRequestDto>,
) -> ServerResult<RestResponseJson<Null>> {
    let department_service = DepartmentService::new(state.db_conn);
    department_service.delete_departments(params.into()).await?;
    Ok(RestResponse::null())
}

/// Update department
#[utoipa::path(
    operation_id = "updateDepartment",
    description = "Update department",
    patch,
    path = "/updateDepartment",
    request_body = UpdateDepartmentRequestDto,
    responses(
        (status = OK, description = "ok", body = RestResponseJson<Null>)
    )
)]
pub async fn update_department(
    State(state): State<AppState>,
    Json(params): Json<UpdateDepartmentRequestDto>,
) -> ServerResult<RestResponseJson<Null>> {
    let department_service = DepartmentService::new(state.db_conn);
    department_service.update_department(params.into()).await?;
    Ok(RestResponse::null())
}
