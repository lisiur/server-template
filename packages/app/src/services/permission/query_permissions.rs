use entity::{
    permissions, relation_groups_permissions, relation_permissions_roles,
    relation_permissions_users,
};
use sea_orm::prelude::*;
use uuid::Uuid;

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

    pub async fn query_permissions_by_user_id(&self, user_id: Uuid) -> AppResult<Vec<Permission>> {
        let permissions = permissions::Entity::find()
            .inner_join(relation_permissions_users::Entity)
            .filter(relation_permissions_users::Column::UserId.eq(user_id))
            .all(&self.0)
            .await?;

        Ok(permissions.into_iter().map(Permission::from).collect())
    }

    pub async fn query_permissions_by_role_id(&self, role_id: Uuid) -> AppResult<Vec<Permission>> {
        let permissions = permissions::Entity::find()
            .inner_join(relation_permissions_roles::Entity)
            .filter(relation_permissions_roles::Column::RoleId.eq(role_id))
            .all(&self.0)
            .await?;

        Ok(permissions.into_iter().map(Permission::from).collect())
    }

    pub async fn query_permissions_by_role_id_list(
        &self,
        role_id_list: Vec<Uuid>,
    ) -> AppResult<Vec<Permission>> {
        let permissions = permissions::Entity::find()
            .inner_join(relation_permissions_roles::Entity)
            .filter(relation_permissions_roles::Column::RoleId.is_in(role_id_list))
            .all(&self.0)
            .await?;

        Ok(permissions.into_iter().map(Permission::from).collect())
    }

    pub async fn query_permissions_by_group_id(
        &self,
        group_id: Uuid,
    ) -> AppResult<Vec<Permission>> {
        let permissions = permissions::Entity::find()
            .inner_join(relation_groups_permissions::Entity)
            .filter(relation_groups_permissions::Column::GroupId.eq(group_id))
            .all(&self.0)
            .await?;

        Ok(permissions.into_iter().map(Permission::from).collect())
    }

    pub async fn query_permissions_by_group_id_list(
        &self,
        group_id_list: Vec<Uuid>,
    ) -> AppResult<Vec<Permission>> {
        let permissions = permissions::Entity::find()
            .inner_join(relation_groups_permissions::Entity)
            .filter(relation_groups_permissions::Column::GroupId.is_in(group_id_list))
            .all(&self.0)
            .await?;

        Ok(permissions.into_iter().map(Permission::from).collect())
    }
}
