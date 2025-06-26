use std::collections::HashMap;

use entity::{
    permissions, relation_permissions_departments, relation_permissions_permission_groups,
    relation_permissions_roles, relation_permissions_user_groups, relation_permissions_users,
};
use sea_orm::{Condition, prelude::*};
use uuid::Uuid;

use crate::{models::permission::Permission, result::AppResult, utils::query::PageableQuery};

use super::PermissionService;

pub struct FilterPermissionsParams {
    pub kind: Option<String>,
}

impl From<FilterPermissionsParams> for Condition {
    fn from(value: FilterPermissionsParams) -> Self {
        Condition::all().add_option(value.kind.map(|kind| permissions::Column::Kind.like(kind)))
    }
}

impl PermissionService {
    pub async fn query_permissions_by_page(
        &self,
        params: PageableQuery<FilterPermissionsParams>,
    ) -> AppResult<(Vec<Permission>, i64)> {
        let (records, count) = self.crud.find_by_condition_with_count(params).await?;

        let records = records.into_iter().map(Permission::from).collect();

        Ok((records, count))
    }

    pub async fn query_permissions_by_user_id(&self, user_id: Uuid) -> AppResult<Vec<Permission>> {
        let permissions = permissions::Entity::find()
            .inner_join(relation_permissions_users::Entity)
            .filter(relation_permissions_users::Column::UserId.eq(user_id))
            .all(&self.conn)
            .await?;

        Ok(permissions.into_iter().map(Permission::from).collect())
    }

    pub async fn query_permissions_by_user_id_list(
        &self,
        user_id_list: Vec<Uuid>,
    ) -> AppResult<HashMap<Uuid, Vec<Permission>>> {
        if user_id_list.is_empty() {
            return Ok(HashMap::new());
        }
        let results = permissions::Entity::find()
            .find_also_related(relation_permissions_users::Entity)
            .filter(relation_permissions_users::Column::UserId.is_in(user_id_list))
            .all(&self.conn)
            .await?;

        let mut map = HashMap::new();
        for (permission, relation) in results {
            let user_id = relation.unwrap().user_id;
            map.entry(user_id)
                .or_insert_with(Vec::new)
                .push(Permission::from(permission));
        }

        Ok(map)
    }

    pub async fn query_permissions_by_role_id(&self, role_id: Uuid) -> AppResult<Vec<Permission>> {
        let permissions = permissions::Entity::find()
            .inner_join(relation_permissions_roles::Entity)
            .filter(relation_permissions_roles::Column::RoleId.eq(role_id))
            .all(&self.conn)
            .await?;

        Ok(permissions.into_iter().map(Permission::from).collect())
    }

    pub async fn query_permissions_by_roles_id_list(
        &self,
        role_id_list: Vec<Uuid>,
    ) -> AppResult<HashMap<Uuid, Vec<Permission>>> {
        if role_id_list.is_empty() {
            return Ok(HashMap::new());
        }
        let results = permissions::Entity::find()
            .find_also_related(relation_permissions_roles::Entity)
            .filter(relation_permissions_roles::Column::RoleId.is_in(role_id_list))
            .all(&self.conn)
            .await?;

        let mut map = HashMap::new();
        for (permission, relation) in results {
            let role_id = relation.unwrap().role_id;
            map.entry(role_id)
                .or_insert_with(Vec::new)
                .push(Permission::from(permission));
        }

        Ok(map)
    }

    pub async fn query_permissions_by_user_group_id(
        &self,
        group_id: Uuid,
    ) -> AppResult<Vec<Permission>> {
        let permissions = permissions::Entity::find()
            .inner_join(relation_permissions_user_groups::Entity)
            .filter(relation_permissions_user_groups::Column::UserGroupId.eq(group_id))
            .all(&self.conn)
            .await?;

        Ok(permissions.into_iter().map(Permission::from).collect())
    }

    pub async fn query_permissions_by_user_groups_id_list(
        &self,
        groups_id_list: Vec<Uuid>,
    ) -> AppResult<HashMap<Uuid, Vec<Permission>>> {
        let results = permissions::Entity::find()
            .find_also_related(relation_permissions_user_groups::Entity)
            .filter(relation_permissions_user_groups::Column::UserGroupId.is_in(groups_id_list))
            .all(&self.conn)
            .await?;

        let mut map = HashMap::new();
        for (permission, relation) in results {
            let user_group_id = relation.unwrap().user_group_id;
            map.entry(user_group_id)
                .or_insert_with(Vec::new)
                .push(Permission::from(permission));
        }

        Ok(map)
    }

    pub async fn query_permissions_by_department_id(
        &self,
        department_id: Uuid,
    ) -> AppResult<Vec<Permission>> {
        let permissions = permissions::Entity::find()
            .inner_join(relation_permissions_departments::Entity)
            .filter(relation_permissions_departments::Column::DepartmentId.eq(department_id))
            .all(&self.conn)
            .await?;

        Ok(permissions.into_iter().map(Permission::from).collect())
    }

    pub async fn query_permissions_by_departments_id_list(
        &self,
        departments_id_list: Vec<Uuid>,
    ) -> AppResult<HashMap<Uuid, Vec<Permission>>> {
        let results = permissions::Entity::find()
            .find_also_related(relation_permissions_departments::Entity)
            .filter(
                relation_permissions_departments::Column::DepartmentId.is_in(departments_id_list),
            )
            .all(&self.conn)
            .await?;

        let mut map = HashMap::new();
        for (permission, relation) in results {
            let department_id = relation.unwrap().department_id;
            map.entry(department_id)
                .or_insert_with(Vec::new)
                .push(Permission::from(permission));
        }

        Ok(map)
    }

    pub async fn query_permissions_by_permission_group_id_list(
        &self,
        permission_group_id_list: Vec<Uuid>,
    ) -> AppResult<HashMap<Uuid, Vec<Permission>>> {
        let results = permissions::Entity::find()
            .find_also_related(relation_permissions_permission_groups::Entity)
            .filter(
                relation_permissions_permission_groups::Column::PermissionGroupId
                    .is_in(permission_group_id_list),
            )
            .all(&self.conn)
            .await?;

        let mut map = HashMap::new();
        for (permission, relation) in results {
            let permission_group_id = relation.unwrap().permission_group_id;
            map.entry(permission_group_id)
                .or_insert_with(Vec::new)
                .push(Permission::from(permission));
        }

        Ok(map)
    }
}
