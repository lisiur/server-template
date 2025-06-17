use std::{cell::RefCell, collections::HashMap, rc::Rc};

use entity::{permissions, relation_groups_permissions, roles};
use sea_orm::prelude::*;
use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    error::AppException,
    models::permission::Permission,
    result::AppResult,
    services::{
        department::DepartmentService, group::GroupService, permission::PermissionService,
        role::RoleService,
    },
};

use super::AuthService;

impl AuthService {
    /// Query user permissions(explicit and implicit) by user id.
    pub async fn query_user_permissions(&self, user_id: Uuid) -> AppResult<Vec<Permission>> {
        let permission_service = PermissionService::new(self.0.clone());
        let role_service = RoleService::new(self.0.clone());
        let group_service = GroupService::new(self.0.clone());
        let department_service = DepartmentService::new(self.0.clone());

        // query user related groups.
        let mut user_related_groups = HashMap::new();
        let user_groups = group_service.query_groups_by_user_id(user_id).await?;
        for user_group in user_groups {
            let groups = group_service.query_group_chain(user_group.id).await?;
            for group in groups {
                user_related_groups.entry(group.id).or_insert(group);
            }
        }
        let user_related_groups = user_related_groups.into_values().collect::<Vec<_>>();
        let user_related_groups_id_list =
            user_related_groups.iter().map(|g| g.id).collect::<Vec<_>>();

        // query user related departments.
        let user_departments = department_service
            .query_departments_by_user_id(user_id)
            .await?;
        let user_departments_id_list = user_departments.iter().map(|g| g.id).collect::<Vec<_>>();

        // query user related roles. includes:
        // - roles of user
        // - roles of user related groups
        // - roles of user departments
        let mut user_related_roles = HashMap::new();

        // roles of user
        let mut user_roles = role_service.query_roles_by_user_id(user_id).await?;

        // roles of user related groups
        let user_related_groups_roles = role_service
            .query_roles_by_groups_id_list(user_related_groups_id_list.clone())
            .await?;

        // roles of user departments
        let user_departments_roles = role_service
            .query_roles_by_departments_id_list(user_departments_id_list.clone())
            .await?;

        // combine them all
        user_roles.extend(user_related_groups_roles);
        user_roles.extend(user_departments_roles);

        for role in user_roles {
            user_related_roles.entry(role.id).or_insert(role);
        }
        let user_related_roles = user_related_roles.into_values().collect::<Vec<_>>();
        let user_related_roles_id_list = user_related_roles
            .iter()
            .map(|role| role.id)
            .collect::<Vec<_>>();

        // query user related permissions. includes:
        // - relations of user
        // - relations of user related roles
        // - relations of user related groups
        // - relations of user departments
        let mut user_related_permissions = HashMap::new();

        // permissions of user
        let mut user_permissions = permission_service
            .query_permissions_by_user_id(user_id)
            .await?;

        // permissions of user related roles
        let user_related_roles_permissions = permission_service
            .query_permissions_by_roles_id_list(user_related_roles_id_list)
            .await?;

        // permissions of user related groups
        let user_related_groups_permissions = permission_service
            .query_permissions_by_groups_id_list(user_related_groups_id_list)
            .await?;

        // permissions of user departments
        let user_departments_permissions = permission_service
            .query_permissions_by_departments_id_list(user_departments_id_list)
            .await?;

        // combine them all
        user_permissions.extend(user_related_roles_permissions);
        user_permissions.extend(user_related_groups_permissions);
        user_permissions.extend(user_departments_permissions);

        for permission in user_permissions {
            user_related_permissions
                .entry(permission.id)
                .or_insert(permission);
        }
        let permissions = user_related_permissions.into_values().collect::<Vec<_>>();

        Ok(permissions)
    }

    /// Query role permissions(explicit and implicit) by role id.
    pub async fn query_role_permissions(&self, role_id: Uuid) -> AppResult<Vec<Permission>> {
        let role = roles::Entity::find_by_id(role_id).one(&self.0).await?;
        let Some(role) = role else {
            return Ok(vec![]);
        };

        let permissions = role.find_related(permissions::Entity).all(&self.0).await?;
        let permissions = permissions.into_iter().map(Permission::from).collect();

        Ok(permissions)
    }

    /// Query department permissions(explicit and implicit) by department id.
    pub async fn query_department_permissions(
        &self,
        department_id: Uuid,
    ) -> AppResult<Vec<Permission>> {
        let role_service = RoleService::new(self.0.clone());
        let permission_service = PermissionService::new(self.0.clone());

        let related_roles = role_service
            .query_roles_by_department_id(department_id)
            .await?;
        let roles_id_list = related_roles.iter().map(|role| role.id).collect::<Vec<_>>();

        let mut permissions = permission_service
            .query_permissions_by_department_id(department_id)
            .await?;
        let roles_permissions = permission_service
            .query_permissions_by_roles_id_list(roles_id_list)
            .await?;
        permissions.extend(roles_permissions);

        Ok(permissions.into_iter().map(Permission::from).collect())
    }

    /// Query group permissions(explicit and implicit) by group id.
    pub async fn query_group_permissions(&self, group_id: Uuid) -> AppResult<Vec<Permission>> {
        let group_service = GroupService::new(self.0.clone());
        let role_service = RoleService::new(self.0.clone());
        let permission_service = PermissionService::new(self.0.clone());
        let groups = group_service.query_group_chain_models(group_id).await?;
        let groups_id_list = groups.iter().map(|group| group.id).collect::<Vec<_>>();
        let related_roles = role_service
            .query_roles_by_groups_id_list(groups_id_list.clone())
            .await?;
        let roles_id_list = related_roles.iter().map(|role| role.id).collect::<Vec<_>>();
        let mut permissions = permission_service
            .query_permissions_by_groups_id_list(groups_id_list)
            .await?;
        let roles_permissions = permission_service
            .query_permissions_by_roles_id_list(roles_id_list)
            .await?;
        permissions.extend(roles_permissions);

        Ok(permissions.into_iter().map(Permission::from).collect())
    }

    /// Query group tree permissions(explicit) by group id.
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

    /// Query group chain permissions(explicit) by group id.
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
