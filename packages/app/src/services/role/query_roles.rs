use crate::models::role::Role;
use entity::{relation_groups_roles, relation_roles_users, roles};
use sea_orm::prelude::*;
use uuid::Uuid;

use crate::{result::AppResult, services::role::RoleService, utils::query::SelectQuery};

impl RoleService {
    pub async fn query_roles_by_page(&self, query: SelectQuery) -> AppResult<(Vec<Role>, i64)> {
        let (roles, count) = query
            .all_with_count::<roles::Model>(roles::Entity, &self.0)
            .await?;

        let roles = roles.into_iter().map(Role::from).collect();

        Ok((roles, count))
    }

    pub async fn query_roles_by_user_id(&self, user_id: Uuid) -> AppResult<Vec<Role>> {
        let roles = roles::Entity::find()
            .inner_join(relation_roles_users::Entity)
            .filter(relation_roles_users::Column::UserId.eq(user_id))
            .all(&self.0)
            .await?;

        Ok(roles.into_iter().map(Role::from).collect())
    }

    pub async fn query_roles_by_group_id(&self, group_id: Uuid) -> AppResult<Vec<Role>> {
        let roles = roles::Entity::find()
            .inner_join(relation_groups_roles::Entity)
            .filter(relation_groups_roles::Column::GroupId.eq(group_id))
            .all(&self.0)
            .await?;

        Ok(roles.into_iter().map(Role::from).collect())
    }

    pub async fn query_roles_by_group_id_list(
        &self,
        group_id_list: Vec<Uuid>,
    ) -> AppResult<Vec<Role>> {
        let roles = roles::Entity::find()
            .inner_join(relation_groups_roles::Entity)
            .filter(relation_groups_roles::Column::GroupId.is_in(group_id_list))
            .all(&self.0)
            .await?;

        Ok(roles.into_iter().map(Role::from).collect())
    }
}
