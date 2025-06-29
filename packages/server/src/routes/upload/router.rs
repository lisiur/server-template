use app::services::upload::UploadService;
use axum::Json;
use utoipa::OpenApi;

use crate::{
    extractors::app_service::AppService,
    init_router,
    response::{ApiResponse, ResponseJson},
    result::ServerResult,
    routes::upload::dto::{MergeFileChunksDto, UploadFileChunkDto},
};

use super::dto::{CreateUploadDto, UploadDto};

#[derive(OpenApi)]
#[openapi(paths(create_upload, upload_file_chunk, merge_file_chunks))]
pub(crate) struct ApiDoc;
init_router!(create_upload, upload_file_chunk, merge_file_chunks);

/// Upload file
#[utoipa::path(
    operation_id = "createUpload",
    post,
    path = "/createUser",
    request_body = CreateUploadDto,
    responses(
        (status = OK, description = "ok", body = ResponseJson<UploadDto>)
    )
)]
pub async fn create_upload(
    upload_service: AppService<UploadService>,
    Json(params): Json<CreateUploadDto>,
) -> ServerResult<ApiResponse> {
    let upload = upload_service.create_upload(params.into()).await?;

    Ok(ApiResponse::json(upload))
}

/// Upload file chunk
#[utoipa::path(
    operation_id = "uploadFileChunk",
    post,
    path = "/uploadFileChunk",
    request_body = UploadFileChunkDto,
    responses(
        (status = OK, description = "ok", body = ResponseJson<UploadDto>)
    )
)]
pub async fn upload_file_chunk(
    upload_service: AppService<UploadService>,
    Json(params): Json<UploadFileChunkDto>,
) -> ServerResult<ApiResponse> {
    let upload = upload_service.create_upload_chunk(params.into()).await?;

    Ok(ApiResponse::json(upload))
}

/// Merge file chunks
#[utoipa::path(
    operation_id = "mergeFileChunks",
    post,
    path = "/mergeFileChunks",
    request_body = MergeFileChunksDto,
    responses(
        (status = OK, description = "ok", body = ResponseJson<UploadDto>)
    )
)]
pub async fn merge_file_chunks(
    upload_service: AppService<UploadService>,
    Json(params): Json<MergeFileChunksDto>,
) -> ServerResult<ApiResponse> {
    let upload = upload_service.merge_upload_chunks(params.upload_id).await?;

    Ok(ApiResponse::json(upload))
}
