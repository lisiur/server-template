use migration::{ColumnRef, IntoColumnRef};
use sea_orm::{Condition, prelude::*};
use serde::{Deserialize, Serialize};
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use utoipa::ToSchema;

use entity::{relation_roles_user_groups, relation_users_user_groups, user_groups};
use uuid::Uuid;

use crate::{
    error::AppException,
    models::user_group::UserGroup,
    result::AppResult,
    services::user_group::UserGroupService,
    utils::query::{Cursor, PageableQuery, TreeQuery},
};

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct UserGroupsFilterParams {
    pub name: Option<String>,
}

impl From<UserGroupsFilterParams> for Condition {
    fn from(value: UserGroupsFilterParams) -> Self {
        Condition::all().add_option(value.name.map(|name| user_groups::Column::Name.like(name)))
    }
}

#[derive(Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub enum UserGroupsOrderField {
    Name,
}

impl From<UserGroupsOrderField> for ColumnRef {
    fn from(value: UserGroupsOrderField) -> Self {
        match value {
            UserGroupsOrderField::Name => user_groups::Column::Name.into_column_ref(),
        }
    }
}

impl UserGroupService {
    pub async fn query_user_groups_by_page(
        &self,
        params: PageableQuery<UserGroupsFilterParams, UserGroupsOrderField>,
    ) -> AppResult<(Vec<UserGroup>, i64)> {
        let (records, total) = self.crud.find_by_condition_with_count(params).await?;
        let groups = records.into_iter().map(UserGroup::from).collect::<Vec<_>>();
        Ok((groups, total))
    }

    pub async fn query_user_group_by_id(&self, id: Uuid) -> AppResult<UserGroup> {
        let group = self.crud.find_by_id(id).await?;

        let Some(group) = group else {
            return Err(AppException::UserGroupNotFound.into());
        };

        Ok(UserGroup::from(group))
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

    pub async fn query_user_group_ancestors(&self, group_id: Uuid) -> AppResult<Vec<UserGroup>> {
        let groups = TreeQuery::new(user_groups::Entity)
            .query_ancestors_with_one(&self.conn, group_id)
            .await?;
        if groups.is_empty() {
            return Err(AppException::UserGroupNotFound.into());
        }

        Ok(groups.into_iter().map(UserGroup::from).collect())
    }

    pub async fn query_many_user_group_ancestors(
        &self,
        group_id_list: Vec<Uuid>,
    ) -> AppResult<Vec<UserGroup>> {
        if group_id_list.is_empty() {
            return Ok(vec![]);
        }

        let groups = TreeQuery::new(user_groups::Entity)
            .query_ancestors_with_many(&self.conn, group_id_list)
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
            .query_descendants_with_one(&self.conn, group_id)
            .await?;

        Ok(groups)
    }

    pub async fn query_user_groups_by_user_id(&self, user_id: Uuid) -> AppResult<Vec<UserGroup>> {
        let groups = user_groups::Entity::find()
            .inner_join(relation_users_user_groups::Entity)
            .filter(relation_users_user_groups::Column::UserId.eq(user_id))
            .all(&self.conn)
            .await?;

        Ok(groups.into_iter().map(UserGroup::from).collect())
    }

    pub async fn query_user_groups_by_user_id_list(
        &self,
        user_id_list: Vec<Uuid>,
    ) -> AppResult<HashMap<Uuid, Vec<UserGroup>>> {
        if user_id_list.is_empty() {
            return Ok(HashMap::new());
        }

        let results = user_groups::Entity::find()
            .find_also_related(relation_users_user_groups::Entity)
            .filter(relation_users_user_groups::Column::UserId.is_in(user_id_list))
            .all(&self.conn)
            .await?;

        let mut map = HashMap::new();
        for (department, relation) in results {
            let user_id = relation.unwrap().user_id;
            map.entry(user_id)
                .or_insert_with(Vec::new)
                .push(UserGroup::from(department));
        }

        Ok(map)
    }

    pub async fn query_user_groups_by_role_id(&self, role_id: Uuid) -> AppResult<Vec<UserGroup>> {
        let groups = user_groups::Entity::find()
            .inner_join(relation_roles_user_groups::Entity)
            .filter(relation_roles_user_groups::Column::RoleId.eq(role_id))
            .all(&self.conn)
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
