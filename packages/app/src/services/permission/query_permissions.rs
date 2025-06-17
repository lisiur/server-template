use std::collections::HashMap;

use entity::{
    permissions, relation_departments_permissions, relation_groups_permissions,
    relation_permissions_roles, relation_permissions_users,
};
use sea_orm::prelude::*;
use uuid::Uuid;

use crate::{
    models::permission::Permission,
    result::AppResult,
    utils::query::{FilterAtom, FilterCondition, PageableQuery, SelectQuery},
};

use super::PermissionService;

impl PermissionService {
    pub async fn query_permissions_by_page<T: PageableQuery<FilterPermissionsParams>>(
        &self,
        params: T,
    ) -> AppResult<(Vec<Permission>, i64)> {
        let mut select_query = SelectQuery::default().with_cursor(params.cursor());
        let filter = params.into_filter();
        if let Some(ref kind) = filter.kind {
            if !kind.is_empty() {
                select_query.add_atom_filter(FilterAtom {
                    field: permissions::Column::Kind.as_str().to_string(),
                    condition: FilterCondition::Like(format!("%{kind}%")),
                });
            }
        }
        let (records, count) = select_query
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

        let flatten_permissions = self.flat_permissions(permissions).await?;

        Ok(flatten_permissions
            .into_iter()
            .map(Permission::from)
            .collect())
    }

    pub async fn query_permissions_by_role_id(&self, role_id: Uuid) -> AppResult<Vec<Permission>> {
        let permissions = permissions::Entity::find()
            .inner_join(relation_permissions_roles::Entity)
            .filter(relation_permissions_roles::Column::RoleId.eq(role_id))
            .all(&self.0)
            .await?;

        let flatten_permissions = self.flat_permissions(permissions).await?;

        Ok(flatten_permissions
            .into_iter()
            .map(Permission::from)
            .collect())
    }

    pub async fn query_permissions_by_roles_id_list(
        &self,
        roles_id_list: Vec<Uuid>,
    ) -> AppResult<Vec<Permission>> {
        let permissions = permissions::Entity::find()
            .inner_join(relation_permissions_roles::Entity)
            .filter(relation_permissions_roles::Column::RoleId.is_in(roles_id_list))
            .all(&self.0)
            .await?;

        let flatten_permissions = self.flat_permissions(permissions).await?;

        Ok(flatten_permissions
            .into_iter()
            .map(Permission::from)
            .collect())
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

        let flatten_permissions = self.flat_permissions(permissions).await?;

        Ok(flatten_permissions
            .into_iter()
            .map(Permission::from)
            .collect())
    }

    pub async fn query_permissions_by_groups_id_list(
        &self,
        groups_id_list: Vec<Uuid>,
    ) -> AppResult<Vec<Permission>> {
        let permissions = permissions::Entity::find()
            .inner_join(relation_groups_permissions::Entity)
            .filter(relation_groups_permissions::Column::GroupId.is_in(groups_id_list))
            .all(&self.0)
            .await?;

        let flatten_permissions = self.flat_permissions(permissions).await?;

        Ok(flatten_permissions
            .into_iter()
            .map(Permission::from)
            .collect())
    }

    pub async fn query_permissions_by_department_id(
        &self,
        department_id: Uuid,
    ) -> AppResult<Vec<Permission>> {
        let permissions = permissions::Entity::find()
            .inner_join(relation_departments_permissions::Entity)
            .filter(relation_departments_permissions::Column::DepartmentId.eq(department_id))
            .all(&self.0)
            .await?;

        let flatten_permissions = self.flat_permissions(permissions).await?;

        Ok(flatten_permissions
            .into_iter()
            .map(Permission::from)
            .collect())
    }

    pub async fn query_permissions_by_departments_id_list(
        &self,
        departments_id_list: Vec<Uuid>,
    ) -> AppResult<Vec<Permission>> {
        let permissions = permissions::Entity::find()
            .inner_join(relation_departments_permissions::Entity)
            .filter(
                relation_departments_permissions::Column::DepartmentId.is_in(departments_id_list),
            )
            .all(&self.0)
            .await?;

        let flatten_permissions = self.flat_permissions(permissions).await?;

        Ok(flatten_permissions
            .into_iter()
            .map(Permission::from)
            .collect())
    }

    async fn flat_permissions(
        &self,
        permissions: Vec<permissions::Model>,
    ) -> AppResult<Vec<permissions::Model>> {
        let (permissions, permission_groups): (Vec<permissions::Model>, Vec<permissions::Model>) =
            permissions.into_iter().partition(|x| x.parent_id.is_some());

        let permission_groups_id_list = permission_groups
            .into_iter()
            .map(|x| x.id)
            .collect::<Vec<_>>();
        let sub_permissions = if permission_groups_id_list.is_empty() {
            vec![]
        } else {
            permissions::Entity::find()
                .filter(permissions::Column::ParentId.is_in(permission_groups_id_list))
                .all(&self.0)
                .await?
        };

        let mut permission_map = HashMap::<Uuid, permissions::Model>::new();
        for role in [permissions, sub_permissions].concat() {
            permission_map.insert(role.id, role);
        }

        Ok(permission_map.into_values().collect())
    }
}

pub struct FilterPermissionsParams {
    pub kind: Option<String>,
}
