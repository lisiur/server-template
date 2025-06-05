use sea_orm::{Statement, prelude::*};
use serde::Serialize;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use utoipa::ToSchema;

use entity::{groups, relation_groups_roles, relation_groups_users};
use uuid::Uuid;

use crate::{
    error::AppException,
    models::group::Group,
    result::AppResult,
    services::group::GroupService,
    utils::query::{Cursor, FilterAtom, FilterCondition, PageableQuery, SelectQuery},
};

impl GroupService {
    pub async fn query_groups_by_page<T: PageableQuery<FilterGroupsParams>>(
        &self,
        params: T,
    ) -> AppResult<(Vec<Group>, i64)> {
        let mut select_query = SelectQuery::default().with_cursor(params.cursor());
        let filter = params.into_filter();
        if let Some(ref name) = filter.name {
            if !name.is_empty() {
                select_query.add_atom_filter(FilterAtom {
                    field: groups::Column::Name.as_str().to_string(),
                    condition: FilterCondition::Like(format!("%{name}%")),
                });
            }
        }
        let (groups, count) = select_query
            .all_with_count::<groups::Model>(groups::Entity, &self.0)
            .await?;
        let groups = groups.into_iter().map(Group::from).collect();

        Ok((groups, count))
    }

    pub async fn query_group_tree(&self, group_id: Uuid) -> AppResult<GroupTree> {
        let groups = self.query_group_tree_models(group_id).await?;
        if groups.is_empty() {
            return Err(AppException::GroupNotFound.into());
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

    pub async fn query_group_chain(&self, group_id: Uuid) -> AppResult<Vec<Group>> {
        let groups = self.query_group_chain_models(group_id).await?;
        if groups.is_empty() {
            return Err(AppException::GroupNotFound.into());
        }

        Ok(groups.into_iter().map(Group::from).collect())
    }

    pub async fn query_group_tree_models(&self, group_id: Uuid) -> AppResult<Vec<groups::Model>> {
        let groups = groups::Entity::find()
            .from_raw_sql(Statement::from_sql_and_values(
                self.0.get_database_backend(),
                format!(
                    r#"
                        WITH RECURSIVE group_tree AS (
                            SELECT * FROM {table} WHERE {id} = $1
                            UNION ALL
                            SELECT g.* FROM {table} g
                            JOIN group_tree gt ON g.{parent_id} = gt.{id}
                        )
                        SELECT * FROM group_tree
                    "#,
                    table = entity::groups::Entity.as_str(),
                    id = entity::groups::Column::Id.as_str(),
                    parent_id = entity::groups::Column::ParentId.as_str(),
                ),
                vec![group_id.into()],
            ))
            .all(&self.0)
            .await?;
        Ok(groups)
    }

    pub async fn query_group_chain_models(&self, group_id: Uuid) -> AppResult<Vec<groups::Model>> {
        let groups = groups::Entity::find()
            .from_raw_sql(Statement::from_sql_and_values(
                self.0.get_database_backend(),
                format!(
                    r#"
                        WITH RECURSIVE group_chain AS (
                            SELECT * FROM {table} WHERE {id} = $1
                            UNION ALL
                            SELECT g.* FROM {table} g
                            JOIN group_chain gc ON g.{id} = gc.{parent_id}
                        )
                        SELECT * FROM group_chain
                    "#,
                    table = entity::groups::Entity.as_str(),
                    id = entity::groups::Column::Id.as_str(),
                    parent_id = entity::groups::Column::ParentId.as_str(),
                ),
                vec![group_id.into()],
            ))
            .all(&self.0)
            .await?;

        Ok(groups)
    }

    pub async fn query_groups_by_user_id(&self, user_id: Uuid) -> AppResult<Vec<Group>> {
        let groups = groups::Entity::find()
            .inner_join(relation_groups_users::Entity)
            .filter(relation_groups_users::Column::UserId.eq(user_id))
            .all(&self.0)
            .await?;

        Ok(groups.into_iter().map(Group::from).collect())
    }

    pub async fn query_groups_by_role_id(&self, role_id: Uuid) -> AppResult<Vec<Group>> {
        let groups = groups::Entity::find()
            .inner_join(relation_groups_roles::Entity)
            .filter(relation_groups_roles::Column::RoleId.eq(role_id))
            .all(&self.0)
            .await?;

        Ok(groups.into_iter().map(Group::from).collect())
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
