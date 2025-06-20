use sea_orm::prelude::*;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use entity::{permission_groups, relation_permission_groups_users};
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
        let permission_groups = self
            .query_permission_group_tree_models(permission_group_id)
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

    pub async fn query_permission_group_tree_models(
        &self,
        permission_group_id: Uuid,
    ) -> AppResult<Vec<permission_groups::Model>> {
        let permission_groups = TreeQuery::new(permission_groups::Entity)
            .query_descendants_with_one(&self.0, permission_group_id)
            .await?;
        Ok(permission_groups)
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
