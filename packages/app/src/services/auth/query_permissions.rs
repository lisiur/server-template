use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};

use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    models::{
        department::Department, permission::Permission, permission_group::PermissionGroup,
        role::Role, role_group::RoleGroup, user::User, user_group::UserGroup,
    },
    result::AppResult,
    services::{
        department::DepartmentService, permission::PermissionService,
        permission_group::PermissionGroupService, role::RoleService, role_group::RoleGroupService,
        user::UserService, user_group::UserGroupService,
    },
};

use super::AuthService;

fn distinct_permission(permissions: Vec<Arc<Mutex<Permission>>>) -> Vec<Arc<Mutex<Permission>>> {
    let mut permission_map = HashMap::new();
    for permission in permissions {
        let id = permission.lock().unwrap().id;
        permission_map.entry(id).or_insert(permission.clone());
    }
    permission_map.into_values().collect()
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssignedPermissionGroupPermissions {
    pub permission_group: PermissionGroup,
    pub permissions: Vec<Arc<Mutex<Permission>>>,
    pub children: HashMap<Uuid, Arc<Mutex<AssignedPermissionGroupPermissions>>>,
}

impl AssignedPermissionGroupPermissions {
    pub fn flatten_permissions(&self) -> Vec<Arc<Mutex<Permission>>> {
        let mut permissions = self.permissions.clone();

        for child in self.children.values() {
            permissions.extend(child.lock().unwrap().flatten_permissions());
        }

        distinct_permission(permissions)
    }
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssignedRolePermissions {
    pub role: Role,
    pub permissions: Vec<Arc<Mutex<Permission>>>,
    pub permission_groups: Vec<Arc<Mutex<AssignedPermissionGroupPermissions>>>,
}

impl AssignedRolePermissions {
    pub fn flatten_permissions(&self) -> Vec<Arc<Mutex<Permission>>> {
        let mut permissions = self.permissions.clone();

        for permission_group in self.permission_groups.iter() {
            permissions.extend(permission_group.lock().unwrap().flatten_permissions());
        }

        distinct_permission(permissions)
    }
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssignedRoleGroupPermissions {
    pub role_group: RoleGroup,
    pub roles: Vec<Arc<Mutex<AssignedRolePermissions>>>,
    pub children: HashMap<Uuid, Arc<Mutex<AssignedRoleGroupPermissions>>>,
}

impl AssignedRoleGroupPermissions {
    pub fn flatten_permissions(&self) -> Vec<Arc<Mutex<Permission>>> {
        let mut permissions = vec![];

        for role in &self.roles {
            permissions.extend(role.lock().unwrap().flatten_permissions());
        }

        for child in self.children.values() {
            permissions.extend(child.lock().unwrap().flatten_permissions());
        }

        distinct_permission(permissions)
    }
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssignedUserGroupPermissions {
    pub user_group: UserGroup,
    pub permissions: Vec<Arc<Mutex<Permission>>>,
    pub inherited_group: Option<Arc<Mutex<AssignedUserGroupPermissions>>>,
    pub permission_groups: Vec<Arc<Mutex<AssignedPermissionGroupPermissions>>>,
    pub roles: Vec<Arc<Mutex<AssignedRolePermissions>>>,
    pub role_groups: Vec<Arc<Mutex<AssignedRoleGroupPermissions>>>,
}

impl AssignedUserGroupPermissions {
    pub fn flatten_permissions(&self) -> Vec<Arc<Mutex<Permission>>> {
        let mut permissions = self.permissions.clone();

        if let Some(inherited_group) = &self.inherited_group {
            permissions.extend(inherited_group.lock().unwrap().flatten_permissions());
        }

        for permission_group in &self.permission_groups {
            permissions.extend(permission_group.lock().unwrap().flatten_permissions());
        }

        for role in &self.roles {
            permissions.extend(role.lock().unwrap().flatten_permissions());
        }

        for role_group in &self.role_groups {
            permissions.extend(role_group.lock().unwrap().flatten_permissions());
        }

        distinct_permission(permissions)
    }
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssignedDepartmentPermissions {
    pub department: Department,
    pub permissions: Vec<Arc<Mutex<Permission>>>,
    pub permission_groups: Vec<Arc<Mutex<AssignedPermissionGroupPermissions>>>,
    pub roles: Vec<Arc<Mutex<AssignedRolePermissions>>>,
    pub role_groups: Vec<Arc<Mutex<AssignedRoleGroupPermissions>>>,
}

impl AssignedDepartmentPermissions {
    pub fn flatten_permissions(&self) -> Vec<Arc<Mutex<Permission>>> {
        let mut permissions = self.permissions.clone();

        for permission_group in &self.permission_groups {
            permissions.extend(permission_group.lock().unwrap().flatten_permissions());
        }

        for role in &self.roles {
            permissions.extend(role.lock().unwrap().flatten_permissions());
        }

        for role_group in &self.role_groups {
            permissions.extend(role_group.lock().unwrap().flatten_permissions());
        }

        distinct_permission(permissions)
    }
}

#[derive(Serialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct AssignedUserPermissions {
    pub user: User,
    pub departments: Vec<Arc<Mutex<AssignedDepartmentPermissions>>>,
    pub user_groups: Vec<Arc<Mutex<AssignedUserGroupPermissions>>>,
    pub role_groups: Vec<Arc<Mutex<AssignedRoleGroupPermissions>>>,
    pub roles: Vec<Arc<Mutex<AssignedRolePermissions>>>,
    pub permission_groups: Vec<Arc<Mutex<AssignedPermissionGroupPermissions>>>,
    pub permissions: Vec<Arc<Mutex<Permission>>>,
}

impl AssignedUserPermissions {
    pub fn flatten_permissions(&self) -> Vec<Arc<Mutex<Permission>>> {
        let mut permissions = self.permissions.clone();

        for permission_group in &self.permission_groups {
            permissions.extend(permission_group.lock().unwrap().flatten_permissions());
        }

        for role in &self.roles {
            permissions.extend(role.lock().unwrap().flatten_permissions());
        }

        for role_group in &self.role_groups {
            permissions.extend(role_group.lock().unwrap().flatten_permissions());
        }

        for user_group in &self.user_groups {
            permissions.extend(user_group.lock().unwrap().flatten_permissions());
        }

        for department in &self.departments {
            permissions.extend(department.lock().unwrap().flatten_permissions());
        }

        distinct_permission(permissions)
    }
}

pub struct PermissionTree {
    users_map: HashMap<Uuid, Arc<Mutex<AssignedUserPermissions>>>,
    departments_map: HashMap<Uuid, Arc<Mutex<AssignedDepartmentPermissions>>>,
    user_groups_map: HashMap<Uuid, Arc<Mutex<AssignedUserGroupPermissions>>>,
    role_groups_map: HashMap<Uuid, Arc<Mutex<AssignedRoleGroupPermissions>>>,
    roles_map: HashMap<Uuid, Arc<Mutex<AssignedRolePermissions>>>,
    permission_groups_map: HashMap<Uuid, Arc<Mutex<AssignedPermissionGroupPermissions>>>,
    #[allow(dead_code)]
    permissions_map: HashMap<Uuid, Arc<Mutex<Permission>>>,
}

#[derive(Clone, Copy)]
pub enum PermissionTreeEntry {
    UserId(Uuid),
    PermissionGroupId(Uuid),
    RoleId(Uuid),
    RoleGroupId(Uuid),
    UserGroupId(Uuid),
    DepartmentId(Uuid),
}

impl AuthService {
    async fn query_permission_tree(&self, entry: PermissionTreeEntry) -> AppResult<PermissionTree> {
        let user_service = UserService::new(self.0.clone());
        let permission_service = PermissionService::new(self.0.clone());
        let role_service = RoleService::new(self.0.clone());
        let user_group_service = UserGroupService::new(self.0.clone());
        let department_service = DepartmentService::new(self.0.clone());
        let role_group_service = RoleGroupService::new(self.0.clone());
        let permission_group_service = PermissionGroupService::new(self.0.clone());

        let mut users_map = HashMap::<Uuid, Arc<Mutex<AssignedUserPermissions>>>::new();
        let mut permissions_map = HashMap::<Uuid, Arc<Mutex<Permission>>>::new();
        let mut permission_groups_map =
            HashMap::<Uuid, Arc<Mutex<AssignedPermissionGroupPermissions>>>::new();
        let mut roles_map = HashMap::<Uuid, Arc<Mutex<AssignedRolePermissions>>>::new();
        let mut role_groups_map = HashMap::<Uuid, Arc<Mutex<AssignedRoleGroupPermissions>>>::new();
        let mut user_groups_map = HashMap::<Uuid, Arc<Mutex<AssignedUserGroupPermissions>>>::new();
        let mut departments_map = HashMap::<Uuid, Arc<Mutex<AssignedDepartmentPermissions>>>::new();

        match entry {
            PermissionTreeEntry::UserId(user_id) => {
                let user = user_service.query_user_by_id(user_id).await?;
                let user_permissions = AssignedUserPermissions {
                    user,
                    permissions: vec![],
                    permission_groups: vec![],
                    roles: vec![],
                    role_groups: vec![],
                    user_groups: vec![],
                    departments: vec![],
                };
                users_map.insert(user_id, Arc::new(Mutex::new(user_permissions)));
            }
            PermissionTreeEntry::PermissionGroupId(permission_group_id) => {
                let permission_group = permission_group_service
                    .query_permission_group_by_id(permission_group_id)
                    .await?;
                let permission_group_permissions = AssignedPermissionGroupPermissions {
                    permission_group,
                    permissions: vec![],
                    children: HashMap::new(),
                };
                permission_groups_map.insert(
                    permission_group_id,
                    Arc::new(Mutex::new(permission_group_permissions)),
                );
            }
            PermissionTreeEntry::RoleId(role_id) => {
                let role = role_service.query_role_by_id(role_id).await?;
                let role_permissions = AssignedRolePermissions {
                    role,
                    permissions: vec![],
                    permission_groups: vec![],
                };
                roles_map.insert(role_id, Arc::new(Mutex::new(role_permissions)));
            }
            PermissionTreeEntry::RoleGroupId(role_group_id) => {
                let role_group = role_group_service
                    .query_role_group_by_id(role_group_id)
                    .await?;
                let role_group_permissions = AssignedRoleGroupPermissions {
                    role_group,
                    roles: vec![],
                    children: HashMap::new(),
                };
                role_groups_map.insert(role_group_id, Arc::new(Mutex::new(role_group_permissions)));
            }
            PermissionTreeEntry::UserGroupId(user_group_id) => {
                let user_group = user_group_service
                    .query_user_group_by_id(user_group_id)
                    .await?;
                let user_group_permissions = AssignedUserGroupPermissions {
                    user_group,
                    inherited_group: None,
                    permissions: vec![],
                    permission_groups: vec![],
                    roles: vec![],
                    role_groups: vec![],
                };
                user_groups_map.insert(user_group_id, Arc::new(Mutex::new(user_group_permissions)));
            }
            PermissionTreeEntry::DepartmentId(department_id) => {
                let department = department_service
                    .query_department_by_id(department_id)
                    .await?;
                let department_permissions = AssignedDepartmentPermissions {
                    department,
                    permissions: vec![],
                    permission_groups: vec![],
                    roles: vec![],
                    role_groups: vec![],
                };
                departments_map.insert(department_id, Arc::new(Mutex::new(department_permissions)));
            }
        };

        // 1. query user departments
        let user_id_list = users_map.keys().cloned().collect::<Vec<_>>();
        let departments = department_service
            .query_departments_by_user_id_list(user_id_list)
            .await?;

        departments.into_iter().for_each(|(user_id, departments)| {
            let department_id_list = departments.iter().map(|x| x.id).collect::<Vec<_>>();
            departments.into_iter().for_each(|department| {
                departments_map
                    .entry(department.id)
                    .or_insert(Arc::new(Mutex::new(AssignedDepartmentPermissions {
                        department,
                        permissions: vec![],
                        permission_groups: vec![],
                        roles: vec![],
                        role_groups: vec![],
                    })));
            });
            // fill user departments
            users_map
                .get_mut(&user_id)
                .unwrap()
                .lock()
                .unwrap()
                .departments = department_id_list
                .into_iter()
                .map(|department_id| departments_map.get(&department_id).unwrap().clone())
                .collect();
        });

        // 2.1 query user_groups
        let user_id_list = users_map.keys().cloned().collect::<Vec<_>>();
        let user_groups = user_group_service
            .query_user_groups_by_user_id_list(user_id_list)
            .await?;

        user_groups.into_iter().for_each(|(user_id, user_groups)| {
            let user_group_id_list = user_groups.iter().map(|x| x.id).collect::<Vec<_>>();
            user_groups.into_iter().for_each(|user_group| {
                user_groups_map
                    .entry(user_group.id)
                    .or_insert(Arc::new(Mutex::new(AssignedUserGroupPermissions {
                        user_group,
                        permissions: vec![],
                        permission_groups: vec![],
                        roles: vec![],
                        role_groups: vec![],
                        inherited_group: None,
                    })));
            });
            // fill user groups
            users_map
                .get_mut(&user_id)
                .unwrap()
                .lock()
                .unwrap()
                .user_groups = user_group_id_list
                .into_iter()
                .map(|user_group_id| user_groups_map.get(&user_group_id).unwrap().clone())
                .collect();
        });
        // query all user group ancestors
        let user_group_id_list = user_groups_map.keys().cloned().collect::<Vec<_>>();
        let user_group_ancestors = user_group_service
            .query_many_user_group_ancestors(user_group_id_list)
            .await?;
        let mut user_group_ancestors = user_group_ancestors
            .into_iter()
            .map(|x| {
                Arc::new(Mutex::new(AssignedUserGroupPermissions {
                    user_group: x,
                    inherited_group: None,
                    permissions: vec![],
                    permission_groups: vec![],
                    roles: vec![],
                    role_groups: vec![],
                }))
            })
            .collect::<Vec<_>>();
        // record to user_groups_map
        for user_group in user_group_ancestors.iter() {
            let id = user_group.lock().unwrap().user_group.id;
            user_groups_map.entry(id).or_insert(user_group.clone());
        }
        // assign user_group's inherited_group
        for user_group in user_group_ancestors.iter_mut() {
            let id = user_group.lock().unwrap().user_group.id;
            let parent_id = user_group.lock().unwrap().user_group.parent_id;
            if let Some(parent_id) = parent_id {
                let parent_group = user_groups_map.get(&parent_id).unwrap();
                let user_group = user_groups_map.get(&id).unwrap();
                user_group.lock().unwrap().inherited_group = Some(parent_group.clone());
            }
        }

        // 3.1 query user role_groups
        let user_id_list = users_map.keys().cloned().collect::<Vec<_>>();
        let role_groups = role_group_service
            .query_role_groups_by_user_id_list(user_id_list)
            .await?;

        role_groups.into_iter().for_each(|(user_id, role_groups)| {
            let role_group_id_list = role_groups.iter().map(|x| x.id).collect::<Vec<_>>();
            role_groups.into_iter().for_each(|role_group| {
                role_groups_map
                    .entry(role_group.id)
                    .or_insert(Arc::new(Mutex::new(AssignedRoleGroupPermissions {
                        role_group,
                        roles: vec![],
                        children: HashMap::new(),
                    })));
            });
            // fill user role_groups
            users_map
                .get_mut(&user_id)
                .unwrap()
                .lock()
                .unwrap()
                .role_groups = role_group_id_list
                .into_iter()
                .map(|role_group_id| role_groups_map.get(&role_group_id).unwrap().clone())
                .collect();
        });

        // 3.2 query department role_groups
        let department_id_list = departments_map
            .values()
            .map(|x| x.lock().unwrap().department.id)
            .collect::<Vec<_>>();
        let department_role_groups = role_group_service
            .query_role_groups_by_department_id_list(department_id_list)
            .await?;
        department_role_groups
            .into_iter()
            .for_each(|(dept_id, role_groups)| {
                let role_groups_id_list = role_groups.iter().map(|x| x.id).collect::<Vec<_>>();
                // record role groups map
                role_groups.into_iter().for_each(|role_group| {
                    role_groups_map
                        .entry(role_group.id)
                        .or_insert(Arc::new(Mutex::new(AssignedRoleGroupPermissions {
                            role_group,
                            roles: vec![],
                            children: HashMap::new(),
                        })));
                });
                // fill department
                departments_map
                    .get_mut(&dept_id)
                    .unwrap()
                    .lock()
                    .unwrap()
                    .role_groups = role_groups_id_list
                    .into_iter()
                    .map(|role_group_id| role_groups_map.get(&role_group_id).unwrap().clone())
                    .collect();
            });

        // 3.3 query user_group role_groups
        let user_group_id_list = user_groups_map
            .values()
            .map(|x| x.lock().unwrap().user_group.id)
            .collect::<Vec<_>>();
        let user_groups_role_groups = role_group_service
            .query_role_groups_by_user_group_id_list(user_group_id_list)
            .await?;
        user_groups_role_groups
            .into_iter()
            .for_each(|(user_group_id, role_groups)| {
                let role_groups_id_list = role_groups.iter().map(|x| x.id).collect::<Vec<_>>();
                // record role groups map
                role_groups.into_iter().for_each(|role_group| {
                    role_groups_map
                        .entry(role_group.id)
                        .or_insert(Arc::new(Mutex::new(AssignedRoleGroupPermissions {
                            role_group,
                            roles: vec![],
                            children: HashMap::new(),
                        })));
                });
                // fill department
                user_groups_map
                    .get_mut(&user_group_id)
                    .unwrap()
                    .lock()
                    .unwrap()
                    .role_groups = role_groups_id_list
                    .into_iter()
                    .map(|role_group_id| role_groups_map.get(&role_group_id).unwrap().clone())
                    .collect();
            });
        // 3.4 query role_groups sub role_groups
        let role_group_id_list = role_groups_map.keys().cloned().collect::<Vec<_>>();
        let role_groups = role_group_service
            .query_role_groups_by_ancestors(role_group_id_list)
            .await?;
        role_groups.into_iter().for_each(|role_group| {
            role_groups_map
                .entry(role_group.id)
                .or_insert(Arc::new(Mutex::new(AssignedRoleGroupPermissions {
                    role_group,
                    roles: vec![],
                    children: HashMap::new(),
                })));
        });
        // fill role_group children
        let mut parent_child_pairs = Vec::new();
        for role_group in role_groups_map.values() {
            let id = role_group.lock().unwrap().role_group.id;
            let parent_id = role_group.lock().unwrap().role_group.parent_id;
            if let Some(parent_id) = parent_id {
                parent_child_pairs.push((id, parent_id));
            }
        }
        for (id, parent_id) in parent_child_pairs {
            let child = role_groups_map.get(&id).unwrap().clone();
            let parent = role_groups_map.get_mut(&parent_id).unwrap();
            parent.lock().unwrap().children.insert(id, child);
        }

        // 4.1 query user roles
        let user_id_list = users_map.keys().cloned().collect::<Vec<_>>();
        let roles = role_service
            .query_roles_by_user_id_list(user_id_list)
            .await?;

        roles.into_iter().for_each(|(user_id, roles)| {
            let role_id_list = roles.iter().map(|x| x.id).collect::<Vec<_>>();
            roles.into_iter().for_each(|role| {
                roles_map
                    .entry(role.id)
                    .or_insert(Arc::new(Mutex::new(AssignedRolePermissions {
                        role,
                        permissions: vec![],
                        permission_groups: vec![],
                    })));
            });
            // fill user roles
            users_map.get_mut(&user_id).unwrap().lock().unwrap().roles = role_id_list
                .into_iter()
                .map(|role_id| roles_map.get(&role_id).unwrap().clone())
                .collect();
        });

        // 4.2 query department roles
        let department_id_list = departments_map
            .keys()
            .map(|x| x.clone())
            .collect::<Vec<_>>();
        let roles = role_service
            .query_roles_by_department_id_list(department_id_list)
            .await?;
        roles.into_iter().for_each(|(department_id, roles)| {
            let role_id_list = roles.iter().map(|role| role.id).collect::<Vec<_>>();
            roles.into_iter().for_each(|role| {
                roles_map
                    .entry(role.id)
                    .or_insert(Arc::new(Mutex::new(AssignedRolePermissions {
                        role,
                        permissions: vec![],
                        permission_groups: vec![],
                    })));
            });
            // fill roles
            departments_map
                .get_mut(&department_id)
                .unwrap()
                .lock()
                .unwrap()
                .roles = role_id_list
                .into_iter()
                .map(|role_id| roles_map.get(&role_id).unwrap().clone())
                .collect();
        });

        // 4.3 query user_group roles
        let user_group_id_list = user_groups_map
            .keys()
            .map(|x| x.clone())
            .collect::<Vec<_>>();
        let roles = role_service
            .query_roles_by_user_groups_id_list(user_group_id_list)
            .await?;
        roles.into_iter().for_each(|(user_group_id, roles)| {
            let role_id_list = roles.iter().map(|role| role.id).collect::<Vec<_>>();
            roles.into_iter().for_each(|role| {
                roles_map
                    .entry(role.id)
                    .or_insert(Arc::new(Mutex::new(AssignedRolePermissions {
                        role,
                        permissions: vec![],
                        permission_groups: vec![],
                    })));
            });
            // fill roles
            user_groups_map
                .get_mut(&user_group_id)
                .unwrap()
                .lock()
                .unwrap()
                .roles = role_id_list
                .into_iter()
                .map(|role_id| roles_map.get(&role_id).unwrap().clone())
                .collect();
        });

        // 4.4 query role_group roles
        let role_group_id_list = role_groups_map
            .keys()
            .map(|x| x.clone())
            .collect::<Vec<_>>();
        let roles = role_service
            .query_roles_by_role_groups_id_list(role_group_id_list)
            .await?;
        roles.into_iter().for_each(|(role_group_id, roles)| {
            let role_id_list = roles.iter().map(|role| role.id).collect::<Vec<_>>();
            roles.into_iter().for_each(|role| {
                roles_map
                    .entry(role.id)
                    .or_insert(Arc::new(Mutex::new(AssignedRolePermissions {
                        role,
                        permissions: vec![],
                        permission_groups: vec![],
                    })));
            });
            // fill role groups
            role_groups_map
                .get_mut(&role_group_id)
                .unwrap()
                .lock()
                .unwrap()
                .roles = role_id_list
                .into_iter()
                .map(|role_id| roles_map.get(&role_id).unwrap().clone())
                .collect();
        });

        // 5.1 query user permission_groups
        let user_id_list = users_map.keys().cloned().collect::<Vec<_>>();
        let permission_groups = permission_group_service
            .query_permission_groups_by_user_id_list(user_id_list)
            .await?;

        permission_groups
            .into_iter()
            .for_each(|(user_id, permission_groups)| {
                let permission_group_id_list =
                    permission_groups.iter().map(|x| x.id).collect::<Vec<_>>();
                permission_groups.into_iter().for_each(|permission_group| {
                    permission_groups_map
                        .entry(permission_group.id)
                        .or_insert(Arc::new(Mutex::new(AssignedPermissionGroupPermissions {
                            permission_group,
                            permissions: vec![],
                            children: HashMap::new(),
                        })));
                });
                // fill user permission_groups
                users_map
                    .get_mut(&user_id)
                    .unwrap()
                    .lock()
                    .unwrap()
                    .permission_groups = permission_group_id_list
                    .into_iter()
                    .map(|permission_group_id| {
                        permission_groups_map
                            .get(&permission_group_id)
                            .unwrap()
                            .clone()
                    })
                    .collect();
            });

        // 5.2 query department permission_groups
        let department_id_list = departments_map
            .values()
            .map(|x| x.lock().unwrap().department.id)
            .collect::<Vec<_>>();
        let permission_groups = permission_group_service
            .query_permission_groups_by_department_id_list(department_id_list)
            .await?;
        permission_groups
            .into_iter()
            .for_each(|(department_id, permission_groups)| {
                let permission_group_id_list = permission_groups
                    .iter()
                    .map(|permission_group| permission_group.id)
                    .collect::<Vec<_>>();
                permission_groups.into_iter().for_each(|permission_group| {
                    permission_groups_map
                        .entry(permission_group.id)
                        .or_insert(Arc::new(Mutex::new(AssignedPermissionGroupPermissions {
                            permission_group,
                            permissions: Vec::new(),
                            children: HashMap::new(),
                        })));
                });
                // fill departments
                departments_map
                    .get_mut(&department_id)
                    .unwrap()
                    .lock()
                    .unwrap()
                    .permission_groups = permission_group_id_list
                    .into_iter()
                    .map(|permission_group_id| {
                        permission_groups_map
                            .get(&permission_group_id)
                            .unwrap()
                            .clone()
                    })
                    .collect();
            });

        // 5.3 query user_group permission_groups
        let user_group_id_list = user_groups_map
            .values()
            .map(|x| x.lock().unwrap().user_group.id)
            .collect::<Vec<_>>();
        let permission_groups = permission_group_service
            .query_permission_groups_by_user_group_id_list(user_group_id_list)
            .await?;
        permission_groups
            .into_iter()
            .for_each(|(user_group_id, permission_groups)| {
                let permission_group_id_list = permission_groups
                    .iter()
                    .map(|permission_group| permission_group.id)
                    .collect::<Vec<_>>();
                permission_groups.into_iter().for_each(|permission_group| {
                    permission_groups_map
                        .entry(permission_group.id)
                        .or_insert(Arc::new(Mutex::new(AssignedPermissionGroupPermissions {
                            permission_group,
                            permissions: Vec::new(),
                            children: HashMap::new(),
                        })));
                });
                // fill user_groups
                user_groups_map
                    .get_mut(&user_group_id)
                    .unwrap()
                    .lock()
                    .unwrap()
                    .permission_groups = permission_group_id_list
                    .into_iter()
                    .map(|permission_group_id| {
                        permission_groups_map
                            .get(&permission_group_id)
                            .unwrap()
                            .clone()
                    })
                    .collect();
            });

        // 5.4 query role permission_groups
        let role_id_list = roles_map
            .values()
            .map(|x| x.lock().unwrap().role.id)
            .collect::<Vec<_>>();
        let permission_groups = permission_group_service
            .query_permission_groups_by_role_id_list(role_id_list)
            .await?;
        permission_groups
            .into_iter()
            .for_each(|(role_id, permission_groups)| {
                let permission_group_id_list = permission_groups
                    .iter()
                    .map(|permission_group| permission_group.id)
                    .collect::<Vec<_>>();
                permission_groups.into_iter().for_each(|permission_group| {
                    permission_groups_map
                        .entry(permission_group.id)
                        .or_insert(Arc::new(Mutex::new(AssignedPermissionGroupPermissions {
                            permission_group,
                            permissions: Vec::new(),
                            children: HashMap::new(),
                        })));
                });
                // fill user_groups
                roles_map
                    .get_mut(&role_id)
                    .unwrap()
                    .lock()
                    .unwrap()
                    .permission_groups = permission_group_id_list
                    .into_iter()
                    .map(|permission_group_id| {
                        permission_groups_map
                            .get(&permission_group_id)
                            .unwrap()
                            .clone()
                    })
                    .collect();
            });
        // 5.5 query permission_group sub permission_groups
        let permission_group_id_list = permission_groups_map.keys().cloned().collect::<Vec<_>>();
        let permission_groups = permission_group_service
            .query_permission_groups_by_ancestors(permission_group_id_list)
            .await?;
        permission_groups.into_iter().for_each(|permission_group| {
            permission_groups_map
                .entry(permission_group.id)
                .or_insert(Arc::new(Mutex::new(AssignedPermissionGroupPermissions {
                    permission_group,
                    permissions: vec![],
                    children: HashMap::new(),
                })));
        });
        // fill permission_group children
        let mut parent_child_pairs = Vec::new();
        for permission_group in permission_groups_map.values() {
            let id = permission_group.lock().unwrap().permission_group.id;
            let parent_id = permission_group.lock().unwrap().permission_group.parent_id;
            if let Some(parent_id) = parent_id {
                parent_child_pairs.push((id, parent_id));
            }
        }
        for (id, parent_id) in parent_child_pairs {
            let child = permission_groups_map.get(&id).unwrap().clone();
            let parent = permission_groups_map.get_mut(&parent_id).unwrap();
            parent.lock().unwrap().children.insert(id, child);
        }

        // 6.1 query user permissions
        let user_id_list = users_map.keys().cloned().collect::<Vec<_>>();
        let permissions = permission_service
            .query_permissions_by_user_id_list(user_id_list)
            .await?;

        permissions.into_iter().for_each(|(user_id, permissions)| {
            let permission_id_list = permissions.iter().map(|x| x.id).collect::<Vec<_>>();
            permissions.into_iter().for_each(|permission| {
                permissions_map
                    .entry(permission.id)
                    .or_insert(Arc::new(Mutex::new(permission)));
            });
            // fill user permissions
            users_map
                .get_mut(&user_id)
                .unwrap()
                .lock()
                .unwrap()
                .permissions = permission_id_list
                .into_iter()
                .map(|permission_id| permissions_map.get(&permission_id).unwrap().clone())
                .collect();
        });

        // 6.2 query department permissions
        let department_id_list = departments_map
            .values()
            .map(|x| x.lock().unwrap().department.id)
            .collect::<Vec<_>>();
        let permissions = permission_service
            .query_permissions_by_departments_id_list(department_id_list)
            .await?;
        permissions
            .into_iter()
            .for_each(|(department_id, permissions)| {
                let permission_id_list = permissions
                    .iter()
                    .map(|permission| permission.id)
                    .collect::<Vec<_>>();
                permissions.into_iter().for_each(|permission| {
                    permissions_map
                        .entry(permission.id)
                        .or_insert(Arc::new(Mutex::new(permission)));
                });
                // fill departments
                departments_map
                    .get_mut(&department_id)
                    .unwrap()
                    .lock()
                    .unwrap()
                    .permissions = permission_id_list
                    .into_iter()
                    .map(|permission_id| permissions_map.get(&permission_id).unwrap().clone())
                    .collect();
            });
        // 6.3 query user_group permissions
        let user_group_id_list = user_groups_map
            .values()
            .map(|x| x.lock().unwrap().user_group.id)
            .collect::<Vec<_>>();
        let permissions = permission_service
            .query_permissions_by_user_groups_id_list(user_group_id_list)
            .await?;
        permissions
            .into_iter()
            .for_each(|(user_group_id, permissions)| {
                let permission_id_list = permissions
                    .iter()
                    .map(|permission| permission.id)
                    .collect::<Vec<_>>();
                permissions.into_iter().for_each(|permission| {
                    permissions_map
                        .entry(permission.id)
                        .or_insert(Arc::new(Mutex::new(permission)));
                });
                // fill user_groups
                user_groups_map
                    .get_mut(&user_group_id)
                    .unwrap()
                    .lock()
                    .unwrap()
                    .permissions = permission_id_list
                    .into_iter()
                    .map(|permission_id| permissions_map.get(&permission_id).unwrap().clone())
                    .collect();
            });
        // 6.4 query role permissions
        let role_id_list = roles_map
            .values()
            .map(|x| x.lock().unwrap().role.id)
            .collect::<Vec<_>>();
        let permissions = permission_service
            .query_permissions_by_roles_id_list(role_id_list)
            .await?;
        permissions.into_iter().for_each(|(role_id, permissions)| {
            let permission_id_list = permissions
                .iter()
                .map(|permission| permission.id)
                .collect::<Vec<_>>();
            permissions.into_iter().for_each(|permission| {
                permissions_map
                    .entry(permission.id)
                    .or_insert(Arc::new(Mutex::new(permission)));
            });
            // fill roles
            roles_map
                .get_mut(&role_id)
                .unwrap()
                .lock()
                .unwrap()
                .permissions = permission_id_list
                .into_iter()
                .map(|permission_id| permissions_map.get(&permission_id).unwrap().clone())
                .collect();
        });
        // 6.5 query permission_group permissions
        let permission_group_id_list = permission_groups_map
            .values()
            .map(|x| x.lock().unwrap().permission_group.id)
            .collect::<Vec<_>>();
        let permissions = permission_service
            .query_permissions_by_permission_group_id_list(permission_group_id_list)
            .await?;
        permissions
            .into_iter()
            .for_each(|(permission_group_id, permissions)| {
                let permission_id_list = permissions
                    .iter()
                    .map(|permission| permission.id)
                    .collect::<Vec<_>>();
                permissions.into_iter().for_each(|permission| {
                    permissions_map
                        .entry(permission.id)
                        .or_insert(Arc::new(Mutex::new(permission)));
                });
                // fill permission_groups
                permission_groups_map
                    .get_mut(&permission_group_id)
                    .unwrap()
                    .lock()
                    .unwrap()
                    .permissions = permission_id_list
                    .into_iter()
                    .map(|permission_id| permissions_map.get(&permission_id).unwrap().clone())
                    .collect();
            });

        Ok(PermissionTree {
            permissions_map,
            permission_groups_map,
            users_map,
            roles_map,
            role_groups_map,
            user_groups_map,
            departments_map,
        })
    }

    /// Query user permissions(explicit and implicit) by user id.
    pub async fn query_user_permissions(
        &self,
        user_id: Uuid,
    ) -> AppResult<AssignedUserPermissions> {
        let permission_tree = self
            .query_permission_tree(PermissionTreeEntry::UserId(user_id))
            .await?;

        let user_permissions = permission_tree
            .users_map
            .get(&user_id)
            .unwrap()
            .clone()
            .lock()
            .unwrap()
            .clone();

        Ok(user_permissions)
    }

    /// Query role permissions by role id.
    pub async fn query_role_permissions(
        &self,
        role_id: Uuid,
    ) -> AppResult<AssignedRolePermissions> {
        let permission_tree = self
            .query_permission_tree(PermissionTreeEntry::RoleId(role_id))
            .await?;

        let role_permissions = permission_tree
            .roles_map
            .get(&role_id)
            .unwrap()
            .clone()
            .lock()
            .unwrap()
            .clone();

        Ok(role_permissions)
    }

    /// Query department permissions by department id.
    pub async fn query_department_permissions(
        &self,
        department_id: Uuid,
    ) -> AppResult<AssignedDepartmentPermissions> {
        let permission_tree = self
            .query_permission_tree(PermissionTreeEntry::DepartmentId(department_id))
            .await?;

        let department_permissions = permission_tree
            .departments_map
            .get(&department_id)
            .unwrap()
            .clone()
            .lock()
            .unwrap()
            .clone();

        Ok(department_permissions)
    }

    /// Query user group permissions by user_group id.
    pub async fn query_user_group_permissions(
        &self,
        user_group_id: Uuid,
    ) -> AppResult<AssignedUserGroupPermissions> {
        let permission_tree = self
            .query_permission_tree(PermissionTreeEntry::UserGroupId(user_group_id))
            .await?;

        let group_permissions = permission_tree
            .user_groups_map
            .get(&user_group_id)
            .unwrap()
            .clone()
            .lock()
            .unwrap()
            .clone();

        Ok(group_permissions)
    }

    /// Query role_group permissions by role_group id.
    pub async fn query_role_group_permissions(
        &self,
        role_group_id: Uuid,
    ) -> AppResult<AssignedRoleGroupPermissions> {
        let permission_tree = self
            .query_permission_tree(PermissionTreeEntry::RoleGroupId(role_group_id))
            .await?;

        let group_permissions = permission_tree
            .role_groups_map
            .get(&role_group_id)
            .unwrap()
            .clone()
            .lock()
            .unwrap()
            .clone();

        Ok(group_permissions)
    }

    /// Query permission_group permissions by role_group id.
    pub async fn query_permission_group_permissions(
        &self,
        permission_group_id: Uuid,
    ) -> AppResult<AssignedPermissionGroupPermissions> {
        let permission_tree = self
            .query_permission_tree(PermissionTreeEntry::PermissionGroupId(permission_group_id))
            .await?;

        let group_permissions = permission_tree
            .permission_groups_map
            .get(&permission_group_id)
            .unwrap()
            .clone()
            .lock()
            .unwrap()
            .clone();

        Ok(group_permissions)
    }
}

#[derive(ToSchema, Serialize)]
pub struct GroupPermissionChainNode {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub permissions: Vec<GroupPermissionTreePermissionNode>,
}

pub struct GroupPermissionTree(pub Arc<Mutex<GroupPermissionTreeGroupNode>>);

#[derive(Serialize)]
pub struct GroupPermissionTreeGroupNode {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    pub children: Vec<Arc<Mutex<GroupPermissionTreeGroupNode>>>,
    pub permissions: Vec<GroupPermissionTreePermissionNode>,
}

#[derive(ToSchema, Serialize)]
pub struct GroupPermissionTreePermissionNode {
    pub id: Uuid,
    pub code: String,
    pub kind: String,
    pub description: Option<String>,
}
