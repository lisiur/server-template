use sea_orm::{Statement, prelude::*};
use serde::Serialize;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use utoipa::ToSchema;

use entity::groups;
use uuid::Uuid;

use crate::{error::AppException, result::AppResult, services::group::GroupService};

impl GroupService {
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

    pub async fn query_group_chain(&self, group_id: Uuid) -> AppResult<Vec<GroupTreeNode>> {
        let groups = self.query_group_chain_models(group_id).await?;
        if groups.is_empty() {
            return Err(AppException::GroupNotFound.into());
        }

        let mut group_nodes: Vec<GroupTreeNode> = Vec::new();

        for group in &groups {
            let node = GroupTreeNode {
                id: group.id,
                name: group.name.clone(),
                description: group.description.clone(),
                children: vec![],
            };
            group_nodes.push(node);
        }

        Ok(group_nodes)
    }

    pub async fn query_group_tree_models(&self, group_id: Uuid) -> AppResult<Vec<groups::Model>> {
        let groups = groups::Entity::find()
            .from_raw_sql(Statement::from_sql_and_values(
                self.0.get_database_backend(),
                r#"
                WITH RECURSIVE group_tree AS (
                    SELECT * FROM groups WHERE id = $1
                    UNION ALL
                    SELECT g.* FROM groups g
                    JOIN group_tree gt ON g.parent_id = gt.id
                )
                SELECT * FROM group_tree
            "#,
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
                r#"
                WITH RECURSIVE group_chain AS (
                    SELECT * FROM groups WHERE id = $1
                    UNION ALL
                    SELECT g.* FROM groups g
                    JOIN group_chain gc ON g.id = gc.parent_id
                )
                SELECT * FROM group_chain
            "#,
                vec![group_id.into()],
            ))
            .all(&self.0)
            .await?;

        Ok(groups)
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
