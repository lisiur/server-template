use app::utils::query::Cursor;
use serde::Deserialize;
use utoipa::IntoParams;
use validator::Validate;

#[derive(Debug, Clone, Deserialize, Validate)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedQuery<T> {
    #[validate(range(min = 1))]
    pub page: u64,
    #[validate(range(min = 0))]
    pub page_size: u64,
    #[serde(flatten)]
    pub data: T,
}

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

impl<T> PaginatedQuery<T> {
    pub fn cursor(&self) -> Cursor {
        Cursor {
            limit: self.page_size,
            offset: (self.page - 1) * self.page_size,
        }
    }
}
