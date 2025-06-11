use std::rc::Rc;

use app::services::auth::{
    assign_permissions::AssignUserPermissionsParams,
    query_permissions::{GroupPermissionChainNode, GroupPermissionTreeGroupNode},
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(ToSchema, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LoginRequestDto {
    pub account: String,
    pub password: String,
}

#[derive(ToSchema, Serialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct LoginResponseDto {
    pub user_id: Uuid,
    pub account: String,
}

#[derive(ToSchema, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssignUserPermissionsDto {
    user_id: Uuid,
    permission_id_list: Vec<Uuid>,
}

impl From<AssignUserPermissionsDto> for AssignUserPermissionsParams {
    fn from(value: AssignUserPermissionsDto) -> Self {
        Self {
            permission_id_list: value.permission_id_list,
            user_id: value.user_id,
        }
    }
}

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
#[serde(rename_all = "camelCase")]
pub struct QueryUserPermissionsDto {
    pub user_id: Uuid,
}

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
#[serde(rename_all = "camelCase")]
pub struct QueryGroupPermissionsDto {
    pub group_id: Uuid,
}

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
#[serde(rename_all = "camelCase")]
pub struct QueryDepartmentPermissionsDto {
    pub department_id: Uuid,
}

#[derive(Serialize, ToSchema)]
pub struct GroupTreePermissionsDto(pub Rc<std::cell::RefCell<GroupPermissionTreeGroupNode>>);

#[derive(Serialize, ToSchema)]
pub struct GroupChainPermissionsDto(pub Vec<GroupPermissionChainNode>);
