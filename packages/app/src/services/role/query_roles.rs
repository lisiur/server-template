use crate::{
    models::role::Role,
    utils::query::{FilterAtom, FilterCondition, PageableQuery},
};
use entity::{relation_groups_roles, relation_roles_users, roles};
use sea_orm::prelude::*;
use uuid::Uuid;

use crate::{result::AppResult, services::role::RoleService, utils::query::SelectQuery};

impl RoleService {
    pub async fn query_roles_by_page<T: PageableQuery<FilterRolesParams>>(
        &self,
        params: T,
    ) -> AppResult<(Vec<Role>, i64)> {
        let mut select_query = SelectQuery::default().with_cursor(params.cursor());
        let filter = params.into_filter();
        if let Some(ref name) = filter.name {
            if !name.is_empty() {
                select_query.add_atom_filter(FilterAtom {
                    field: roles::Column::Name.as_str().to_string(),
                    condition: FilterCondition::Like(format!("%{name}%")),
                });
            }
        }
        let (roles, count) = select_query
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

pub struct FilterRolesParams {
    pub name: Option<String>,
}
