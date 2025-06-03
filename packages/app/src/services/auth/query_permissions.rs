use std::{cell::RefCell, collections::HashMap, rc::Rc};

use entity::{permissions, relation_groups_permissions, roles, users};
use sea_orm::prelude::*;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    error::AppException, models::permission::Permission, result::AppResult,
    services::group::GroupService,
};

use super::AuthService;

impl AuthService {
    pub async fn query_user_permissions(&self, user_id: Uuid) -> AppResult<Vec<Permission>> {
        let user = users::Entity::find_by_id(user_id).one(&self.0).await?;
        let Some(user) = user else {
            return Ok(vec![]);
        };

        let permissions = user.find_related(permissions::Entity).all(&self.0).await?;
        let permissions = permissions.into_iter().map(Permission::from).collect();

        Ok(permissions)
    }

    pub async fn query_role_permissions(&self, role_id: Uuid) -> AppResult<Vec<Permission>> {
        let role = roles::Entity::find_by_id(role_id).one(&self.0).await?;
        let Some(role) = role else {
            return Ok(vec![]);
        };

        let permissions = role.find_related(permissions::Entity).all(&self.0).await?;
        let permissions = permissions.into_iter().map(Permission::from).collect();

        Ok(permissions)
    }

    pub async fn query_group_tree_permissions(
        &self,
        group_id: Uuid,
    ) -> AppResult<GroupPermissionTree> {
        let group_service = GroupService::new(self.0.clone());
        let groups = group_service.query_group_tree_models(group_id).await?;
        if groups.is_empty() {
            return Err(AppException::GroupNotFound.into());
        }

        let permissions = groups
            .load_many_to_many(
                permissions::Entity,
                relation_groups_permissions::Entity,
                &self.0,
            )
            .await?;

        let mut group_nodes: HashMap<Uuid, Rc<RefCell<GroupPermissionTreeGroupNode>>> =
            HashMap::new();

        for group in &groups {
            let node = Rc::new(RefCell::new(GroupPermissionTreeGroupNode {
                id: group.id,
                name: group.name.clone(),
                description: group.description.clone(),
                children: vec![],
                permissions: vec![],
            }));
            group_nodes.insert(group.id, node);
        }

        for group in &groups {
            if let Some(parent_id) = group.parent_id {
                if let Some(parent_node) = group_nodes.get(&parent_id) {
                    if let Some(child_node) = group_nodes.get(&group.id) {
                        parent_node.borrow_mut().children.push(child_node.clone());
                    }
                }
            }
        }

        for (group, group_permissions) in groups.iter().zip(permissions.iter()) {
            if let Some(group_node) = group_nodes.get(&group.id) {
                let mut node = group_node.borrow_mut();
                node.permissions = group_permissions
                    .iter()
                    .map(|permission| GroupPermissionTreePermissionNode {
                        id: permission.id,
                        code: permission.code.clone(),
                        kind: permission.kind.clone(),
                        description: permission.description.clone(),
                    })
                    .collect();
            }
        }

        let root_node = group_nodes.get(&group_id).unwrap().clone();

        let tree = GroupPermissionTree(root_node);

        Ok(tree)
    }

    pub async fn query_group_chain_permissions(
        &self,
        group_id: Uuid,
    ) -> AppResult<Vec<GroupPermissionChainNode>> {
        let group_service = GroupService::new(self.0.clone());
        let groups = group_service.query_group_chain_models(group_id).await?;
        let permissions = groups
            .load_many_to_many(
                permissions::Entity,
                relation_groups_permissions::Entity,
                &self.0,
            )
            .await?;

        let mut group_nodes: Vec<GroupPermissionChainNode> = Vec::new();

        for group in &groups {
            let node = GroupPermissionChainNode {
                id: group.id,
                name: group.name.clone(),
                description: group.description.clone(),
                permissions: vec![],
            };
            group_nodes.push(node);
        }

        for (group, group_permissions) in group_nodes.iter_mut().zip(permissions.iter()) {
            group.permissions = group_permissions
                .iter()
                .map(|permission| GroupPermissionTreePermissionNode {
                    id: permission.id,
                    code: permission.code.clone(),
                    kind: permission.kind.clone(),
                    description: permission.description.clone(),
                })
                .collect();
        }

        Ok(group_nodes)
    }
}

#[derive(ToSchema, Serialize)]
pub struct GroupPermissionChainNode {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub permissions: Vec<GroupPermissionTreePermissionNode>,
}

pub struct GroupPermissionTree(pub Rc<RefCell<GroupPermissionTreeGroupNode>>);

#[derive(ToSchema, Serialize)]
pub struct GroupPermissionTreeGroupNode {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    #[schema(no_recursion)]
    pub children: Vec<Rc<RefCell<GroupPermissionTreeGroupNode>>>,
    pub permissions: Vec<GroupPermissionTreePermissionNode>,
}

#[derive(ToSchema, Serialize)]
pub struct GroupPermissionTreePermissionNode {
    pub id: Uuid,
    pub code: String,
    pub kind: String,
    pub description: Option<String>,
}
