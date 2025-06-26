use sea_orm::{Condition, prelude::*};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use entity::{
    permission_groups, relation_permission_groups_departments, relation_permission_groups_roles,
    relation_permission_groups_user_groups, relation_permission_groups_users,
};
use uuid::Uuid;

use crate::{
    error::AppException,
    models::permission_group::PermissionGroup,
    result::AppResult,
    services::permission_group::PermissionGroupService,
    utils::query::{Cursor, PageableQuery, TreeQuery},
};

pub struct FilterPermissionGroupsParams {
    pub name: Option<String>,
}

impl From<FilterPermissionGroupsParams> for Condition {
    fn from(value: FilterPermissionGroupsParams) -> Self {
        Condition::all().add_option(
            value
                .name
                .map(|name| permission_groups::Column::Name.like(name)),
        )
    }
}

impl PermissionGroupService {
    pub async fn query_permission_groups_by_page(
        &self,
        params: PageableQuery<FilterPermissionGroupsParams>,
    ) -> AppResult<(Vec<PermissionGroup>, i64)> {
        let (permission_groups, count) = self.crud.find_by_condition_with_count(params).await?;
        let permission_groups = permission_groups
            .into_iter()
            .map(PermissionGroup::from)
            .collect();

        Ok((permission_groups, count))
    }

    pub async fn query_permission_group_by_id(
        &self,
        permission_group_id: Uuid,
    ) -> AppResult<PermissionGroup> {
        let permission_group = permission_groups::Entity::find_by_id(permission_group_id)
            .one(&self.conn)
            .await?
            .ok_or(AppException::PermissionGroupNotFound)?;

        Ok(PermissionGroup::from(permission_group))
    }

    pub async fn query_permission_group_tree(
        &self,
        permission_group_id: Uuid,
    ) -> AppResult<PermissionGroupTree> {
        let permission_groups = TreeQuery::new(permission_groups::Entity)
            .query_descendants_with_one(&self.conn, permission_group_id)
            .await?
            .into_iter()
            .map(|x| {
                Rc::new(RefCell::new(PermissionGroupTreeNode {
                    permission_group: x.into(),
                    children: vec![],
                }))
            })
            .collect::<Vec<_>>();
        if permission_groups.is_empty() {
            return Err(AppException::PermissionGroupNotFound.into());
        }

        let mut permission_group_tree_nodes: HashMap<Uuid, Rc<RefCell<PermissionGroupTreeNode>>> =
            HashMap::new();

        // record all permission_groups
        for permission_group in &permission_groups {
            permission_group_tree_nodes.insert(
                permission_group.borrow().permission_group.id,
                permission_group.clone(),
            );
        }

        // fill permission_group children
        for permission_group in &permission_groups {
            let parent_id = permission_group.borrow().permission_group.parent_id;
            if let Some(parent_id) = parent_id {
                let parent_permission_group = permission_group_tree_nodes.get(&parent_id).unwrap();
                parent_permission_group
                    .borrow_mut()
                    .children
                    .push(permission_group.clone());
            }
        }

        let root_node = permission_group_tree_nodes
            .get(&permission_group_id)
            .unwrap()
            .clone();

        let tree = PermissionGroupTree(root_node);

        Ok(tree)
    }

    pub async fn query_permission_groups_by_user_id(
        &self,
        user_id: Uuid,
    ) -> AppResult<Vec<PermissionGroup>> {
        let groups = permission_groups::Entity::find()
            .inner_join(relation_permission_groups_users::Entity)
            .filter(relation_permission_groups_users::Column::UserId.eq(user_id))
            .all(&self.conn)
            .await?;

        Ok(groups.into_iter().map(PermissionGroup::from).collect())
    }

    pub async fn query_permission_groups_by_user_id_list(
        &self,
        user_id_list: Vec<Uuid>,
    ) -> AppResult<HashMap<Uuid, Vec<PermissionGroup>>> {
        if user_id_list.is_empty() {
            return Ok(HashMap::new());
        }

        let results = permission_groups::Entity::find()
            .find_also_related(relation_permission_groups_users::Entity)
            .filter(relation_permission_groups_users::Column::UserId.is_in(user_id_list))
            .all(&self.conn)
            .await?;

        let mut map = HashMap::new();
        for (permission_group, relation) in results {
            let user_id = relation.unwrap().user_id;
            map.entry(user_id)
                .or_insert_with(Vec::new)
                .push(PermissionGroup::from(permission_group));
        }

        Ok(map)
    }

    pub async fn query_permission_groups_by_department_id(
        &self,
        department_id: Uuid,
    ) -> AppResult<Vec<PermissionGroup>> {
        let groups = permission_groups::Entity::find()
            .inner_join(relation_permission_groups_departments::Entity)
            .filter(relation_permission_groups_departments::Column::DepartmentId.eq(department_id))
            .all(&self.conn)
            .await?;

        Ok(groups.into_iter().map(PermissionGroup::from).collect())
    }

    pub async fn query_permission_groups_by_department_id_list(
        &self,
        department_id_list: Vec<Uuid>,
    ) -> AppResult<HashMap<Uuid, Vec<PermissionGroup>>> {
        if department_id_list.is_empty() {
            return Ok(HashMap::new());
        }

        let results = permission_groups::Entity::find()
            .find_also_related(relation_permission_groups_departments::Entity)
            .filter(
                relation_permission_groups_departments::Column::DepartmentId
                    .is_in(department_id_list),
            )
            .all(&self.conn)
            .await?;

        let mut map = HashMap::new();
        for (permission_group, relation) in results {
            let department_id = relation.unwrap().department_id;
            map.entry(department_id)
                .or_insert_with(Vec::new)
                .push(PermissionGroup::from(permission_group));
        }

        Ok(map)
    }

    pub async fn query_permission_groups_by_user_group_id(
        &self,
        user_group_id: Uuid,
    ) -> AppResult<Vec<PermissionGroup>> {
        let groups = permission_groups::Entity::find()
            .inner_join(relation_permission_groups_user_groups::Entity)
            .filter(relation_permission_groups_user_groups::Column::UserGroupId.eq(user_group_id))
            .all(&self.conn)
            .await?;

        Ok(groups.into_iter().map(PermissionGroup::from).collect())
    }

    pub async fn query_permission_groups_by_user_group_id_list(
        &self,
        user_group_id_list: Vec<Uuid>,
    ) -> AppResult<HashMap<Uuid, Vec<PermissionGroup>>> {
        if user_group_id_list.is_empty() {
            return Ok(HashMap::new());
        }

        let results = permission_groups::Entity::find()
            .find_also_related(relation_permission_groups_user_groups::Entity)
            .filter(
                relation_permission_groups_user_groups::Column::UserGroupId
                    .is_in(user_group_id_list),
            )
            .all(&self.conn)
            .await?;

        let mut map = HashMap::new();
        for (permission_group, relation) in results {
            let user_group_id = relation.unwrap().user_group_id;
            map.entry(user_group_id)
                .or_insert_with(Vec::new)
                .push(PermissionGroup::from(permission_group));
        }

        Ok(map)
    }

    pub async fn query_permission_groups_by_role_id(
        &self,
        role: Uuid,
    ) -> AppResult<Vec<PermissionGroup>> {
        let groups = permission_groups::Entity::find()
            .inner_join(relation_permission_groups_roles::Entity)
            .filter(relation_permission_groups_roles::Column::RoleId.eq(role))
            .all(&self.conn)
            .await?;

        Ok(groups.into_iter().map(PermissionGroup::from).collect())
    }

    pub async fn query_permission_groups_by_role_id_list(
        &self,
        role_id_list: Vec<Uuid>,
    ) -> AppResult<HashMap<Uuid, Vec<PermissionGroup>>> {
        if role_id_list.is_empty() {
            return Ok(HashMap::new());
        }

        let results = permission_groups::Entity::find()
            .find_also_related(relation_permission_groups_roles::Entity)
            .filter(relation_permission_groups_roles::Column::RoleId.is_in(role_id_list))
            .all(&self.conn)
            .await?;

        let mut map = HashMap::new();
        for (permission_group, relation) in results {
            let role_id = relation.unwrap().role_id;
            map.entry(role_id)
                .or_insert_with(Vec::new)
                .push(PermissionGroup::from(permission_group));
        }

        Ok(map)
    }

    pub async fn query_permission_groups_by_ancestors(
        &self,
        permission_group_id_list: Vec<Uuid>,
    ) -> AppResult<Vec<PermissionGroup>> {
        let permission_groups = TreeQuery::new(permission_groups::Entity)
            .query_descendants_with_many(&self.conn, permission_group_id_list)
            .await?;

        Ok(permission_groups
            .into_iter()
            .map(PermissionGroup::from)
            .collect())
    }
}

pub struct PermissionGroupTree(pub Rc<RefCell<PermissionGroupTreeNode>>);

pub struct PermissionGroupTreeNode {
    pub permission_group: PermissionGroup,
    pub children: Vec<Rc<RefCell<PermissionGroupTreeNode>>>,
}

pub struct QueryPermissionGroupsByPageParams {
    pub cursor: Cursor,
    pub name: Option<String>,
}
