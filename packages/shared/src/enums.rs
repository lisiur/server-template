use serde::{Deserialize, Serialize};
use strum::{Display, EnumString};
use utoipa::ToSchema;

#[derive(
    Clone, Debug, PartialEq, Eq, Serialize, Deserialize, EnumString, Display, Default, ToSchema,
)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "snake_case")]
pub enum Gender {
    #[default]
    Unknown,
    Male,
    Female,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, EnumString, Display, ToSchema)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "snake_case")]
pub enum PermissionKind {
    Menu,
    Operation,
    Data,
}

#[derive(
    Clone, Debug, PartialEq, Eq, Serialize, Deserialize, EnumString, Display, ToSchema, Hash,
)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "snake_case")]
pub enum OperationPermission {
    QueryUsers,
    CreateUser,
    UpdateUser,
    DeleteUser,

    QueryRoles,
    CreateRole,
    UpdateRole,
    DeleteRole,

    QueryPermissions,
    CreatePermission,
    UpdatePermission,
    DeletePermission,

    QueryGroups,
    CreateGroup,
    UpdateGroup,
    DeleteGroup,

    QueryDepartments,
    CreateDepartment,
    UpdateDepartment,
    DeleteDepartment,

    AssignUserPermissions,
    QueryUserPermissions,
    QueryGroupPermissions,
    QueryDepartmentPermissions,
    QueryRolePermissions,
    QueryRoleGroupPermissions,
    QueryPermissionGroupPermissions,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize, EnumString, Display, ToSchema)]
#[serde(rename_all = "camelCase")]
#[strum(serialize_all = "snake_case")]
pub enum DataPermission {}
