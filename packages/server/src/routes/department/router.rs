use app::{services::department::DepartmentService, utils::query::DisableOrder};
use axum::{Extension, Json};
use sea_orm::DatabaseConnection;
use shared::enums::OperationPermission;
use utoipa::OpenApi;

use crate::{
    dto::PageableQueryDto,
    extractors::{app_service::AppService, session::Session},
    init_router,
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
init_router!(
    create_department,
    query_departments_by_page,
    delete_departments,
    update_department
);

/// Query departments by page
#[utoipa::path(
    operation_id = "queryDepartmentsByPage",
    description = "Query departments by page",
    post,
    path = "/queryDepartmentsByPage",
    request_body = PageableQueryDto<FilterDepartmentsDto, DisableOrder>,
    responses(
        (status = OK, description = "ok", body = ResponseJson<PaginatedData<DepartmentDto>>)
    )
)]
pub async fn query_departments_by_page(
    session: Session,
    department_service: AppService<DepartmentService>,
    Json(params): Json<PageableQueryDto<FilterDepartmentsDto, DisableOrder>>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::QueryDepartments)?;

    let (records, total) = department_service
        .query_departments_by_page(params.into())
        .await?;
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
    session: Session,
    Extension(conn): Extension<DatabaseConnection>,
    Json(params): Json<CreateDepartmentRequestDto>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::CreateDepartment)?;

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
    session: Session,
    Extension(conn): Extension<DatabaseConnection>,
    Json(params): Json<DeleteDepartmentsRequestDto>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::DeleteDepartment)?;

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
    session: Session,
    Extension(conn): Extension<DatabaseConnection>,
    Json(params): Json<UpdateDepartmentRequestDto>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::UpdateDepartment)?;

    let department_service = DepartmentService::new(conn);
    department_service.update_department(params.into()).await?;
    Ok(ApiResponse::null())
}
