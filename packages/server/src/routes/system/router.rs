use axum::Extension;
use rsa::{RsaPublicKey, pkcs8::EncodePublicKey};
use utoipa::OpenApi;

use crate::{
    init_router,
    response::{ApiResponse, ResponseJson},
    result::ServerResult,
};

#[derive(OpenApi)]
#[openapi(paths(query_system_rsa_public_key))]
pub(crate) struct ApiDoc;
init_router!(query_system_rsa_public_key);

/// Query system rsa public key
#[utoipa::path(
    operation_id = "querySystemRsaPublicKey",
    get,
    path = "/querySystemRsaPublicKey",
    responses(
        (status = OK, description = "ok", body = ResponseJson<String>)
    )
)]
pub async fn query_system_rsa_public_key(
    Extension(public_key): Extension<RsaPublicKey>,
) -> ServerResult<ApiResponse> {
    let public_key_pem = public_key
        .to_public_key_pem(rsa::pkcs8::LineEnding::LF)
        .unwrap();

    Ok(ApiResponse::json(public_key_pem))
}
