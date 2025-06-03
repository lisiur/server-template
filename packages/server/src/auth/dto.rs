use std::rc::Rc;

use app::services::auth::{
    assign_permissions::AssignUserPermissionParams,
    query_permissions::{GroupPermissionChainNode, GroupPermissionTreeGroupNode},
};
use serde::{Deserialize, Serialize};
use utoipa::{IntoParams, ToSchema};
use uuid::Uuid;

#[derive(ToSchema, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AssignUserPermissionDto {
    user_id: Uuid,
    permission_id: Uuid,
}

impl From<AssignUserPermissionDto> for AssignUserPermissionParams {
    fn from(value: AssignUserPermissionDto) -> Self {
        Self {
            permission_id: value.permission_id,
            user_id: value.user_id,
        }
    }
}

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct QueryUserPermissionsDto {
    pub user_id: Uuid,
}

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct QueryGroupTreePermissionsDto {
    pub group_id: Uuid,
}

#[derive(Serialize, ToSchema)]
pub struct GroupTreePermissionsDto(pub Rc<std::cell::RefCell<GroupPermissionTreeGroupNode>>);

#[derive(Deserialize, IntoParams)]
#[into_params(parameter_in = Query)]
pub struct QueryGroupChainPermissionsDto {
    pub group_id: Uuid,
}

#[derive(Serialize, ToSchema)]
pub struct GroupChainPermissionsDto(pub Vec<GroupPermissionChainNode>);
