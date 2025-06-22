use sea_orm::prelude::*;
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
    utils::query::{Cursor, FilterAtom, FilterCondition, PageableQuery, SelectQuery, TreeQuery},
};

impl PermissionGroupService {
    pub async fn query_permission_groups_by_page<T: PageableQuery<FilterPermissionGroupsParams>>(
        &self,
        params: T,
    ) -> AppResult<(Vec<PermissionGroup>, i64)> {
        let mut select_query = SelectQuery::default().with_cursor(params.cursor());
        let filter = params.into_filter();
        if let Some(ref name) = filter.name {
            if !name.is_empty() {
                select_query.add_atom_filter(FilterAtom {
                    field: permission_groups::Column::Name.as_str().to_string(),
                    condition: FilterCondition::Like(format!("%{name}%")),
                });
            }
        }
        let (permission_groups, count) = select_query
            .all_with_count::<permission_groups::Model>(permission_groups::Entity, &self.0)
            .await?;
        let permission_groups = permission_groups
            .into_iter()
            .map(PermissionGroup::from)
            .collect();

        Ok((permission_groups, count))
    }

    pub async fn query_permission_group_tree(
        &self,
        permission_group_id: Uuid,
    ) -> AppResult<PermissionGroupTree> {
        let permission_groups = TreeQuery::new(permission_groups::Entity)
            .query_descendants_with_one(&self.0, permission_group_id)
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
            .all(&self.0)
            .await?;

        Ok(groups.into_iter().map(PermissionGroup::from).collect())
    }

    pub async fn query_permission_groups_by_department_id(
        &self,
        department_id: Uuid,
    ) -> AppResult<Vec<PermissionGroup>> {
        let groups = permission_groups::Entity::find()
            .inner_join(relation_permission_groups_departments::Entity)
            .filter(relation_permission_groups_departments::Column::DepartmentId.eq(department_id))
            .all(&self.0)
            .await?;

        Ok(groups.into_iter().map(PermissionGroup::from).collect())
    }

    pub async fn query_permission_groups_by_department_id_list(
        &self,
        department_id_list: Vec<Uuid>,
    ) -> AppResult<HashMap<Uuid, Vec<PermissionGroup>>> {
        let results = permission_groups::Entity::find()
            .find_also_related(relation_permission_groups_departments::Entity)
            .filter(
                relation_permission_groups_departments::Column::DepartmentId
                    .is_in(department_id_list),
            )
            .all(&self.0)
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
            .all(&self.0)
            .await?;

        Ok(groups.into_iter().map(PermissionGroup::from).collect())
    }

    pub async fn query_permission_groups_by_user_group_id_list(
        &self,
        user_group_id_list: Vec<Uuid>,
    ) -> AppResult<HashMap<Uuid, Vec<PermissionGroup>>> {
        let results = permission_groups::Entity::find()
            .find_also_related(relation_permission_groups_user_groups::Entity)
            .filter(
                relation_permission_groups_user_groups::Column::UserGroupId
                    .is_in(user_group_id_list),
            )
            .all(&self.0)
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
            .all(&self.0)
            .await?;

        Ok(groups.into_iter().map(PermissionGroup::from).collect())
    }

    pub async fn query_permission_groups_by_role_id_list(
        &self,
        role_id_list: Vec<Uuid>,
    ) -> AppResult<HashMap<Uuid, Vec<PermissionGroup>>> {
        let results = permission_groups::Entity::find()
            .find_also_related(relation_permission_groups_roles::Entity)
            .filter(
                relation_permission_groups_roles::Column::RoleId
                    .is_in(role_id_list),
            )
            .all(&self.0)
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
            .query_descendants_with_many(&self.0, permission_group_id_list)
            .await?;

        Ok(permission_groups.into_iter().map(PermissionGroup::from).collect())
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

pub struct FilterPermissionGroupsParams {
    pub name: Option<String>,
}
