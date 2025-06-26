use app::{
    services::account_book::{AccountBookService, create_account_book::CreateAccountBookParams},
    utils::query::PaginatedQuery,
};
use axum::{Extension, Json, extract::Query};
use sea_orm::DatabaseConnection;
use shared::enums::OperationPermission;
use utoipa::OpenApi;

use crate::{
    dto::PaginatedQueryDto,
    extractors::session::Session,
    init_router,
    response::{ApiResponse, Null, PaginatedData, ResponseJson},
    result::ServerResult,
    routes::account_book::dto::{
        AccountBookDto, CreateAccountBookDto, CreateAccountBookResponseDto, FilterAccountBooksDto,
    },
};

#[derive(OpenApi)]
#[openapi(paths(
    create_account_book,
    query_account_books_by_page,
    delete_account_books,
    update_account_book
))]
pub(crate) struct ApiDoc;
init_router!(
    create_account_book,
    query_account_books_by_page,
    delete_account_books,
    update_account_book
);

/// Query account books by page
#[utoipa::path(
    operation_id = "queryAccountBooksByPage",
    description = "Query account books by page",
    get,
    path = "/queryAccountBooksByPage",
    params(PaginatedQueryDto, FilterAccountBooksDto),
    responses(
        (status = OK, description = "ok", body = ResponseJson<PaginatedData<AccountBookDto>>)
    )
)]
pub async fn query_account_books_by_page(
    session: Session,
    Extension(conn): Extension<DatabaseConnection>,
    Query(query): Query<PaginatedQuery<FilterAccountBooksDto>>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::QueryGroups)?;

    let account_book_service = AccountBookService::new(conn);

    let (account_books, total) = account_book_service
        .query_account_books_by_page(query)
        .await?;
    let records = account_books
        .into_iter()
        .map(AccountBookDto::from)
        .collect::<Vec<_>>();

    Ok(ApiResponse::json(PaginatedData { records, total }))
}

/// Create account book
#[utoipa::path(
    operation_id = "createAccountBook",
    description = "Create account book",
    post,
    path = "/createAccountBook",
    request_body = CreateAccountBookDto,
    responses(
        (status = OK, description = "ok", body = ResponseJson<CreateAccountBookResponseDto>)
    )
)]
pub async fn create_account_book(
    session: Session,
    Extension(conn): Extension<DatabaseConnection>,
    Json(params): Json<CreateAccountBookDto>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::CreateGroup)?;

    let account_book_service = AccountBookService::new(conn);

    let group_id = account_book_service
        .create_account_book(params.into())
        .await?;

    Ok(ApiResponse::json(CreateGroupResponseDto(group_id)))
}

/// Delete groups
#[utoipa::path(
    operation_id = "deleteGroups",
    description = "Delete groups",
    delete,
    path = "/deleteGroups",
    request_body = DeleteGroupsRequestDto,
    responses(
        (status = OK, description = "ok", body = ResponseJson<Null>)
    )
)]
pub async fn delete_account_books(
    session: Session,
    Extension(conn): Extension<DatabaseConnection>,
    Json(params): Json<DeleteGroupsRequestDto>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::DeleteGroup)?;

    let account_book_service = AccountBookService::new(conn);
    account_book_service
        .delete_account_books(params.into())
        .await?;
    Ok(ApiResponse::null())
}

/// Update group
#[utoipa::path(
    operation_id = "updateGroup",
    description = "Update group",
    patch,
    path = "/updateGroup",
    request_body = UpdateGroupRequestDto,
    responses(
        (status = OK, description = "ok", body = ResponseJson<Null>)
    )
)]
pub async fn update_account_book(
    session: Session,
    Extension(conn): Extension<DatabaseConnection>,
    Json(params): Json<UpdateGroupRequestDto>,
) -> ServerResult<ApiResponse> {
    session.assert_has_permission(OperationPermission::UpdateGroup)?;

    let account_book_service = AccountBookService::new(conn);
    account_book_service
        .update_account_book(params.into())
        .await?;
    Ok(ApiResponse::null())
}
