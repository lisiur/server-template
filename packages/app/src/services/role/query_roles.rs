use std::collections::HashMap;

use crate::{error::AppException, models::role::Role, utils::query::PageableQuery};
use entity::{
    relation_roles_departments, relation_roles_role_groups, relation_roles_user_groups,
    relation_roles_users, roles,
};
use sea_orm::{Condition, prelude::*};
use uuid::Uuid;

use crate::{result::AppResult, services::role::RoleService};

pub struct FilterRolesParams {
    pub name: Option<String>,
}

impl From<FilterRolesParams> for Condition {
    fn from(value: FilterRolesParams) -> Self {
        Condition::all().add_option(value.name.map(|name| roles::Column::Name.like(name)))
    }
}

impl RoleService {
    pub async fn query_roles_by_page(
        &self,
        params: PageableQuery<FilterRolesParams>,
    ) -> AppResult<(Vec<Role>, i64)> {
        let (records, total) = self.crud.find_by_condition_with_count(params).await?;
        let roles = records.into_iter().map(Role::from).collect();

        Ok((roles, total))
    }

    pub async fn query_role_by_id(&self, id: Uuid) -> AppResult<Role> {
        let role = roles::Entity::find_by_id(id).one(&self.conn).await?;

        let Some(role) = role else {
            return Err(AppException::RoleNotFound.into());
        };

        Ok(Role::from(role))
    }

    /// Query role group's roles
    pub async fn query_roles_by_role_group_id(&self, role_group_id: Uuid) -> AppResult<Vec<Role>> {
        let roles = roles::Entity::find()
            .filter(roles::Column::ParentId.eq(role_group_id))
            .all(&self.conn)
            .await?;

        Ok(roles.into_iter().map(Role::from).collect())
    }

    /// Query user's roles (flatten)
    pub async fn query_roles_by_user_id(&self, user_id: Uuid) -> AppResult<Vec<Role>> {
        let roles = roles::Entity::find()
            .inner_join(relation_roles_users::Entity)
            .filter(relation_roles_users::Column::UserId.eq(user_id))
            .all(&self.conn)
            .await?;

        Ok(roles.into_iter().map(Role::from).collect())
    }

    pub async fn query_roles_by_user_id_list(
        &self,
        user_id_list: Vec<Uuid>,
    ) -> AppResult<HashMap<Uuid, Vec<Role>>> {
        if user_id_list.is_empty() {
            return Ok(HashMap::new());
        }

        let results = roles::Entity::find()
            .find_also_related(relation_roles_users::Entity)
            .filter(relation_roles_users::Column::UserId.is_in(user_id_list))
            .all(&self.conn)
            .await?;

        let mut map = HashMap::new();
        for (role, relation) in results {
            let user_id = relation.unwrap().user_id;
            map.entry(user_id)
                .or_insert_with(Vec::new)
                .push(Role::from(role));
        }

        Ok(map)
    }

    /// Query user_group's roles
    pub async fn query_roles_by_user_group_id(&self, user_group_id: Uuid) -> AppResult<Vec<Role>> {
        let roles = roles::Entity::find()
            .inner_join(relation_roles_user_groups::Entity)
            .filter(relation_roles_user_groups::Column::UserGroupId.eq(user_group_id))
            .all(&self.conn)
            .await?;

        Ok(roles.into_iter().map(Role::from).collect())
    }

    /// Query user_groups's roles
    pub async fn query_roles_by_user_groups_id_list(
        &self,
        user_group_id_list: Vec<Uuid>,
    ) -> AppResult<HashMap<Uuid, Vec<Role>>> {
        let results = roles::Entity::find()
            .find_also_related(relation_roles_user_groups::Entity)
            .filter(relation_roles_user_groups::Column::UserGroupId.is_in(user_group_id_list))
            .all(&self.conn)
            .await?;
        let mut map = HashMap::new();
        for (role, relation) in results {
            let user_group_id = relation.unwrap().user_group_id;
            map.entry(user_group_id)
                .or_insert_with(Vec::new)
                .push(Role::from(role));
        }
        Ok(map)
    }

    /// Query role_groups's roles
    pub async fn query_roles_by_role_groups_id_list(
        &self,
        role_group_id_list: Vec<Uuid>,
    ) -> AppResult<HashMap<Uuid, Vec<Role>>> {
        let results = roles::Entity::find()
            .find_also_related(relation_roles_role_groups::Entity)
            .filter(relation_roles_role_groups::Column::RoleGroupId.is_in(role_group_id_list))
            .all(&self.conn)
            .await?;
        let mut map = HashMap::new();
        for (role, relation) in results {
            let role_group_id = relation.unwrap().role_group_id;
            map.entry(role_group_id)
                .or_insert_with(Vec::new)
                .push(Role::from(role));
        }
        Ok(map)
    }

    /// Query department's roles
    pub async fn query_roles_by_department_id(&self, department_id: Uuid) -> AppResult<Vec<Role>> {
        let roles = roles::Entity::find()
            .inner_join(relation_roles_departments::Entity)
            .filter(relation_roles_departments::Column::DepartmentId.eq(department_id))
            .all(&self.conn)
            .await?;

        Ok(roles.into_iter().map(Role::from).collect())
    }

    /// Query departments's roles
    pub async fn query_roles_by_department_id_list(
        &self,
        department_id_list: Vec<Uuid>,
    ) -> AppResult<HashMap<Uuid, Vec<Role>>> {
        let results = roles::Entity::find()
            .find_also_related(relation_roles_departments::Entity)
            .filter(relation_roles_departments::Column::DepartmentId.is_in(department_id_list))
            .all(&self.conn)
            .await?;

        let mut map = HashMap::new();
        for (role, relation) in results {
            let department_id = relation.unwrap().department_id;
            map.entry(department_id)
                .or_insert_with(Vec::new)
                .push(Role::from(role));
        }
        Ok(map)
    }
}
