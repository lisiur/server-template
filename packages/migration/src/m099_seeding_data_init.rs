use entity::{
    permission_groups, permissions, relation_permission_groups_roles,
    relation_permissions_permission_groups, relation_permissions_roles, relation_role_groups_users,
    relation_roles_users, roles, users,
};
use sea_orm::prelude::*;
use sea_orm_migration::{prelude::*, sea_orm::ActiveValue::Set};
use shared::{
    enums::{Gender, OperationPermission as OP, PermissionKind},
    utils::hash_password,
};
use uuid::Uuid;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        // Create role "admin"
        let admin_role_id = roles::ActiveModel {
            id: Set(Uuid::nil()),
            name: Set("admin".to_string()),
            description: Set(Some("admin".to_string())),
            built_in: Set(true),
            ..Default::default()
        }
        .insert(db)
        .await?
        .id;

        // Create user "admin"
        let admin_user_id = users::ActiveModel {
            id: Set(Uuid::nil()),
            account: Set("admin".to_string()),
            password_digest: Set(Some(hash_password("Admin@132"))),
            gender: Set(Gender::Unknown.to_string()),
            ..Default::default()
        }
        .insert(db)
        .await?
        .id;

        // Assign user "admin" to role "admin"
        relation_roles_users::ActiveModel {
            role_id: Set(admin_role_id),
            user_id: Set(admin_user_id),
            ..Default::default()
        }
        .insert(db)
        .await?;

        // Create permission groups "system"
        let initial_permission_groups = [permission_groups::ActiveModel {
            id: Set(Uuid::nil()),
            name: Set("system".to_string()),
            ..Default::default()
        }];
        let permissions_id_list = permission_groups::Entity::insert_many(initial_permission_groups)
            .exec_with_returning_keys(db)
            .await?;
        let system_permission_group_id = permissions_id_list[0];

        // Assign permission group "system" to role "admin"
        let relations = [relation_permission_groups_roles::ActiveModel {
            permission_group_id: Set(system_permission_group_id),
            role_id: Set(admin_role_id),
            ..Default::default()
        }];
        relation_permission_groups_roles::Entity::insert_many(relations)
            .exec(db)
            .await?;

        // Create preset permissions
        let sys = system_permission_group_id;
        let preset_permissions = [
            (OP::QueryUsers, sys),
            (OP::CreateUser, sys),
            (OP::UpdateUser, sys),
            (OP::DeleteUser, sys),
            (OP::QueryRoles, sys),
            (OP::CreateRole, sys),
            (OP::UpdateRole, sys),
            (OP::DeleteRole, sys),
            (OP::QueryGroups, sys),
            (OP::CreateGroup, sys),
            (OP::UpdateGroup, sys),
            (OP::DeleteGroup, sys),
            (OP::QueryDepartments, sys),
            (OP::CreateDepartment, sys),
            (OP::UpdateDepartment, sys),
            (OP::DeleteDepartment, sys),
            (OP::AssignUserPermissions, sys),
            (OP::QueryUserPermissions, sys),
            (OP::QueryGroupPermissions, sys),
            (OP::QueryDepartmentPermissions, sys),
            (OP::QueryRolePermissions, sys),
            (OP::QueryRoleGroupPermissions, sys),
            (OP::QueryPermissionGroupPermissions, sys),
        ];

        let permissions_id_list =
            permissions::Entity::insert_many(preset_permissions.iter().map(|x| {
                permissions::ActiveModel {
                    id: Set(Uuid::new_v4()),
                    code: Set(x.0.to_string()),
                    kind: Set(PermissionKind::Operation.to_string()),
                    description: Set(None),
                    built_in: Set(true),
                    ..Default::default()
                }
            }))
            .exec_with_returning_keys(db)
            .await?;

        relation_permissions_permission_groups::Entity::insert_many(
            permissions_id_list.into_iter().map(|x| {
                relation_permissions_permission_groups::ActiveModel {
                    permission_id: Set(x),
                    permission_group_id: Set(system_permission_group_id),
                    ..Default::default()
                }
            }),
        )
        .exec(db)
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        relation_roles_users::Entity::delete_many().exec(db).await?;
        relation_permissions_roles::Entity::delete_many()
            .exec(db)
            .await?;
        relation_permissions_permission_groups::Entity::delete_many()
            .exec(db)
            .await?;
        relation_role_groups_users::Entity::delete_many()
            .exec(db)
            .await?;
        permissions::Entity::delete_many().exec(db).await?;
        users::Entity::delete_many().exec(db).await?;
        roles::Entity::delete_many().exec(db).await?;
        Ok(())
    }
}
