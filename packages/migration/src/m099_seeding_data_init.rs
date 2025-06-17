use entity::{permissions, relation_permissions_roles, relation_roles_users, roles, users};
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

        // Create role group "*"
        let root_role_group_id = roles::ActiveModel {
            id: Set(Uuid::nil()),
            name: Set("*".to_string()),
            description: Set(Some("All".to_string())),
            built_in: Set(true),
            ..Default::default()
        }
        .insert(db)
        .await?
        .id;

        // Create role "admin"
        let admin_role_id = roles::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set("admin".to_string()),
            description: Set(Some("admin".to_string())),
            built_in: Set(true),
            parent_id: Set(Some(root_role_group_id)),
            ..Default::default()
        }
        .insert(db)
        .await?
        .id;

        // Create role "user"
        let user_role_id = roles::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set("user".to_string()),
            description: Set(Some("user".to_string())),
            built_in: Set(true),
            parent_id: Set(Some(root_role_group_id)),
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

        // Create user "test"
        let test_user_id = users::ActiveModel {
            id: Set(Uuid::new_v4()),
            account: Set("test".to_string()),
            password_digest: Set(Some(hash_password("Test@132"))),
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

        // Assign user "test" to role "user"
        relation_roles_users::ActiveModel {
            role_id: Set(user_role_id),
            user_id: Set(test_user_id),
            ..Default::default()
        }
        .insert(db)
        .await?;

        // Create permission groups "*" "basic" "system"
        let initial_permission_groups = [
            permissions::ActiveModel {
                id: Set(Uuid::nil()),
                code: Set("*".to_string()),
                kind: Set(PermissionKind::Operation.to_string()),
                built_in: Set(true),
                ..Default::default()
            },
            permissions::ActiveModel {
                id: Set(Uuid::new_v4()),
                code: Set("basic".to_string()),
                kind: Set(PermissionKind::Operation.to_string()),
                built_in: Set(true),
                ..Default::default()
            },
            permissions::ActiveModel {
                id: Set(Uuid::new_v4()),
                code: Set("system".to_string()),
                kind: Set(PermissionKind::Operation.to_string()),
                built_in: Set(true),
                ..Default::default()
            },
        ];
        let permissions_id_list = permissions::Entity::insert_many(initial_permission_groups)
            .exec_with_returning_keys(db)
            .await?;
        let basic_permission_group_id = permissions_id_list[1];
        let system_permission_group_id = permissions_id_list[2];

        // Assign permission group "basic" to role "user" & "admin"
        // Assign permission group "system" to role "admin"
        let relations = [
            relation_permissions_roles::ActiveModel {
                permission_id: Set(basic_permission_group_id),
                role_id: Set(user_role_id),
                ..Default::default()
            },
            relation_permissions_roles::ActiveModel {
                permission_id: Set(basic_permission_group_id),
                role_id: Set(admin_role_id),
                ..Default::default()
            },
            relation_permissions_roles::ActiveModel {
                permission_id: Set(system_permission_group_id),
                role_id: Set(admin_role_id),
                ..Default::default()
            },
        ];
        relation_permissions_roles::Entity::insert_many(relations)
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
        ];

        permissions::Entity::insert_many(preset_permissions.iter().map(|x| {
            permissions::ActiveModel {
                id: Set(Uuid::new_v4()),
                parent_id: Set(Some(x.1)),
                code: Set(x.0.to_string()),
                kind: Set(PermissionKind::Operation.to_string()),
                description: Set(Some("All".to_string())),
                built_in: Set(true),
                ..Default::default()
            }
        }))
        .exec_with_returning_keys(db)
        .await?;

        Ok(())
    }

    async fn down(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();

        relation_roles_users::Entity::delete_many().exec(db).await?;
        relation_permissions_roles::Entity::delete_many()
            .exec(db)
            .await?;
        permissions::Entity::delete_many().exec(db).await?;
        users::Entity::delete_many().exec(db).await?;
        roles::Entity::delete_many().exec(db).await?;
        Ok(())
    }
}
