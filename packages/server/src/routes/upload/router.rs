use app::{
    App,
    error::{AppError, AppException},
    services::upload::{UploadService, create_upload::CreateUploadChunkParams},
};
use axum::{
    Extension, Json,
    extract::{Multipart, Query},
};
use bytes::Bytes;
use http::{
    HeaderValue,
    header::{CONTENT_DISPOSITION, CONTENT_TYPE},
};
use shared::utils::encode_url;
use utoipa::OpenApi;
use uuid::Uuid;

use crate::{
    error::ServerExceptionCode,
    extractors::app_service::AppService,
    init_router,
    response::{ApiResponse, Null, ResponseJson},
    result::ServerResult,
    routes::upload::dto::{GetUploadDto, MergeFileChunksDto, UploadFileChunkDto},
};

use super::dto::{CreateUploadDto, UploadDto};

#[derive(OpenApi)]
#[openapi(paths(get_file, create_upload, upload_file_chunk, merge_file_chunks))]
pub(crate) struct ApiDoc;
init_router!(
    get_file,
    create_upload,
    upload_file_chunk,
    merge_file_chunks
);

/// Get file
#[utoipa::path(
    operation_id = "getFile",
    get,
    path = "/getFile",
    params(GetUploadDto),
    responses(
        (status = OK, description = "ok", body = ResponseJson<UploadDto>)
    )
)]
pub async fn get_file(
    upload_service: AppService<UploadService>,
    Extension(app): Extension<App>,
    Query(params): Query<GetUploadDto>,
) -> ServerResult<ApiResponse> {
    let upload = upload_service.query_upload_by_id(params.id).await?;

    let Some(upload) = upload else {
        return Err(ServerExceptionCode::NotFound.into());
    };

    let file_path = upload.file_path(&app.upload_dir);
    let Ok(file) = tokio::fs::File::open(file_path).await else {
        return Err(ServerExceptionCode::NotFound.into());
    };

    let mut response = ApiResponse::stream(file);
    response.append_header(
        CONTENT_TYPE,
        HeaderValue::from_str(&format!("{}", upload.mime_type())).unwrap(),
    );
    response.append_header(
        CONTENT_DISPOSITION,
        HeaderValue::from_str(&format!(
            "attachment; filename*=UTF-8''{}",
            encode_url(&upload.name)
        ))
        .unwrap(),
    );

    Ok(response)
}

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
    let missing_chunks = upload_service.query_missing_chunks(upload.id).await?;
    let mut upload: UploadDto = upload.into();
    upload.missing_chunks = missing_chunks;

    Ok(ApiResponse::json(upload))
}

/// Upload file chunk
#[utoipa::path(
    operation_id = "uploadFileChunk",
    post,
    path = "/uploadFileChunk",
    request_body(content = UploadFileChunkDto, content_type = "multipart/form-data"),
    responses(
        (status = OK, description = "ok", body = ResponseJson<Null>)
    )
)]
pub async fn upload_file_chunk(
    upload_service: AppService<UploadService>,
    mut params: Multipart,
) -> ServerResult<ApiResponse> {
    let mut upload_id: Option<Uuid> = None;
    let mut index: Option<i32> = None;
    let mut chunk: Option<Bytes> = None;

    while let Some(field) = params.next_field().await? {
        let field_name = field.name();

        match &field_name {
            Some("uploadId") => {
                let uuid = field
                    .text()
                    .await
                    .map_err(|_| AppError::exception(AppException::InvalidUUID))?;
                let uuid = Uuid::parse_str(&uuid)?;
                upload_id = Some(uuid);
            }
            Some("index") => {
                let num = field.text().await?;
                let num = num
                    .parse::<i32>()
                    .map_err(|_| AppError::exception(AppException::InvalidI32))?;
                index = Some(num);
            }
            Some("chunk") => {
                let bytes = field
                    .bytes()
                    .await
                    .map_err(|_| AppError::exception(AppException::InvalidBytes))?;
                chunk = Some(bytes);
            }
            _ => (),
        }
    }
    let upload_id = upload_id.ok_or(AppError::exception(AppException::MissingField(
        "uploadId".to_string(),
    )))?;
    let index = index.ok_or(AppError::exception(AppException::MissingField(
        "index".to_string(),
    )))?;
    let chunk = chunk.ok_or(AppError::exception(AppException::MissingField(
        "chunk".to_string(),
    )))?;
    let upload_file_chunk_dto = CreateUploadChunkParams {
        upload_id,
        index,
        chunk: chunk.to_vec(),
    };

    upload_service
        .create_upload_chunk(upload_file_chunk_dto.into())
        .await?;

    Ok(ApiResponse::null())
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
    let upload_dto = UploadDto::from(upload);

    Ok(ApiResponse::json(upload_dto))
}
