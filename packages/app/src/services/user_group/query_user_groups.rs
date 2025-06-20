use sea_orm::prelude::*;
use serde::Serialize;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use utoipa::ToSchema;

use entity::{relation_roles_user_groups, relation_users_user_groups, user_groups};
use uuid::Uuid;

use crate::{
    error::AppException,
    models::user_group::UserGroup,
    result::AppResult,
    services::user_group::UserGroupService,
    utils::query::{Cursor, FilterAtom, FilterCondition, PageableQuery, SelectQuery, TreeQuery},
};

impl UserGroupService {
    pub async fn query_groups_by_page<T: PageableQuery<FilterGroupsParams>>(
        &self,
        params: T,
    ) -> AppResult<(Vec<UserGroup>, i64)> {
        let mut select_query = SelectQuery::default().with_cursor(params.cursor());
        let filter = params.into_filter();
        if let Some(ref name) = filter.name {
            if !name.is_empty() {
                select_query.add_atom_filter(FilterAtom {
                    field: user_groups::Column::Name.as_str().to_string(),
                    condition: FilterCondition::Like(format!("%{name}%")),
                });
            }
        }
        let (groups, count) = select_query
            .all_with_count::<user_groups::Model>(user_groups::Entity, &self.0)
            .await?;
        let groups = groups.into_iter().map(UserGroup::from).collect();

        Ok((groups, count))
    }

    pub async fn query_group_tree(&self, group_id: Uuid) -> AppResult<GroupTree> {
        let groups = self.query_group_tree_models(group_id).await?;
        if groups.is_empty() {
            return Err(AppException::UserGroupNotFound.into());
        }

        let mut group_nodes: HashMap<Uuid, Rc<RefCell<GroupTreeNode>>> = HashMap::new();

        for group in &groups {
            let node = Rc::new(RefCell::new(GroupTreeNode {
                id: group.id,
                name: group.name.clone(),
                description: group.description.clone(),
                children: vec![],
            }));
            group_nodes.insert(group.id, node);
        }

        let root_node = group_nodes.get(&group_id).unwrap().clone();

        let tree = GroupTree(root_node);

        Ok(tree)
    }

    pub async fn query_group_ancestors(&self, group_id: Uuid) -> AppResult<Vec<UserGroup>> {
        let groups = self.query_user_group_ancestors_models(group_id).await?;
        if groups.is_empty() {
            return Err(AppException::UserGroupNotFound.into());
        }

        Ok(groups.into_iter().map(UserGroup::from).collect())
    }

    pub async fn query_many_user_group_ancestors(
        &self,
        group_id_list: Vec<Uuid>,
    ) -> AppResult<Vec<UserGroup>> {
        let groups = self
            .query_many_user_group_ancestors_models(group_id_list)
            .await?;
        if groups.is_empty() {
            return Err(AppException::UserGroupNotFound.into());
        }

        Ok(groups.into_iter().map(UserGroup::from).collect())
    }

    pub async fn query_group_tree_models(
        &self,
        group_id: Uuid,
    ) -> AppResult<Vec<user_groups::Model>> {
        let groups = TreeQuery::new(user_groups::Entity)
            .query_descendants_with_one(&self.0, group_id)
            .await?;

        Ok(groups)
    }

    pub async fn query_user_group_ancestors_models(
        &self,
        group_id: Uuid,
    ) -> AppResult<Vec<user_groups::Model>> {
        let groups = TreeQuery::new(user_groups::Entity)
            .query_ancestors_with_one(&self.0, group_id)
            .await?;

        Ok(groups)
    }

    pub async fn query_many_user_group_ancestors_models(
        &self,
        group_id_list: Vec<Uuid>,
    ) -> AppResult<Vec<user_groups::Model>> {
        let groups = TreeQuery::new(user_groups::Entity)
            .query_ancestors_with_many(&self.0, group_id_list)
            .await?;

        Ok(groups)
    }

    pub async fn query_user_groups_by_user_id(&self, user_id: Uuid) -> AppResult<Vec<UserGroup>> {
        let groups = user_groups::Entity::find()
            .inner_join(relation_users_user_groups::Entity)
            .filter(relation_users_user_groups::Column::UserId.eq(user_id))
            .all(&self.0)
            .await?;

        Ok(groups.into_iter().map(UserGroup::from).collect())
    }

    pub async fn query_groups_by_role_id(&self, role_id: Uuid) -> AppResult<Vec<UserGroup>> {
        let groups = user_groups::Entity::find()
            .inner_join(relation_roles_user_groups::Entity)
            .filter(relation_roles_user_groups::Column::RoleId.eq(role_id))
            .all(&self.0)
            .await?;

        Ok(groups.into_iter().map(UserGroup::from).collect())
    }
}

pub struct GroupTree(pub Rc<RefCell<GroupTreeNode>>);

#[derive(ToSchema, Serialize)]
pub struct GroupTreeNode {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    #[schema(no_recursion)]
    pub children: Vec<Rc<RefCell<GroupTreeNode>>>,
}

pub struct QueryGroupsByPageParams {
    pub cursor: Cursor,
    pub name: Option<String>,
}

pub struct FilterGroupsParams {
    pub name: Option<String>,
}
