use serde::Deserialize;
use utoipa::IntoParams;

#[derive(Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
#[allow(dead_code)]
pub struct PaginatedQueryDto {
    #[param(minimum = 1)]
    pub page: u64,
    #[param(minimum = 0)]
    pub page_size: u64,
}
