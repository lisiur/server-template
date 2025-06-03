use entity::permissions;

use crate::{models::permission::Permission, result::AppResult, utils::query::SelectQuery};

use super::PermissionService;

impl PermissionService {
    pub async fn query_permissions_by_page(
        &self,
        query: SelectQuery,
    ) -> AppResult<(Vec<Permission>, i64)> {
        let (records, count) = query
            .all_with_count::<permissions::Model>(permissions::Entity, &self.0)
            .await?;

        let records = records.into_iter().map(Permission::from).collect();

        Ok((records, count))
    }
}
