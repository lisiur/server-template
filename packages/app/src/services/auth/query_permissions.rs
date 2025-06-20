use std::{cell::RefCell, collections::{HashMap, HashSet}, hash::Hash, rc::Rc};

use serde::Serialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::{
    models::{
        department::Department, permission::Permission, permission_group::PermissionGroup,
        role::Role, role_group::RoleGroup, user_group::UserGroup,
    },
    result::AppResult,
    services::{
        department::DepartmentService, permission::PermissionService,
        permission_group::PermissionGroupService, role::RoleService, role_group::RoleGroupService,
        user_group::UserGroupService,
    },
};

use super::AuthService;

pub struct AssignedPermissionGroup {
    pub permission_group: PermissionGroup,
    pub permissions: Vec<Rc<RefCell<Permission>>>,
    pub children: HashMap<Uuid, Rc<RefCell<AssignedPermissionGroup>>>,
}

pub struct AssignedRole {
    pub role: Role,
    pub permissions: Vec<Rc<RefCell<Permission>>>,
    pub permission_groups: Vec<Rc<RefCell<AssignedPermissionGroup>>>,
}

pub struct AssignedRoleGroup {
    pub role_group: RoleGroup,
    pub roles: Vec<Rc<RefCell<AssignedRole>>>,
    pub children: HashMap<Uuid, Rc<RefCell<AssignedRoleGroup>>>,
}

pub struct AssignedUserGroup {
    pub user_group: UserGroup,
    pub inherited_group: Option<Rc<RefCell<AssignedUserGroup>>>,
    pub permissions: Vec<Rc<RefCell<Permission>>>,
    pub permission_groups: Vec<Rc<RefCell<AssignedPermissionGroup>>>,
    pub roles: Vec<Rc<RefCell<AssignedRole>>>,
    pub role_groups: Vec<Rc<RefCell<AssignedRoleGroup>>>,
}

pub struct AssignedDepartment {
    pub department: Department,
    pub permissions: Vec<Rc<RefCell<Permission>>>,
    pub permission_groups: Vec<Rc<RefCell<AssignedPermissionGroup>>>,
    pub roles: Vec<Rc<RefCell<AssignedRole>>>,
    pub role_groups: Vec<Rc<RefCell<AssignedRoleGroup>>>,
}

pub struct UserPermissions {
    pub permissions: Vec<Rc<RefCell<Permission>>>,
    pub permission_groups: Vec<Rc<RefCell<AssignedPermissionGroup>>>,
    pub roles: Vec<Rc<RefCell<AssignedRole>>>,
    pub role_groups: Vec<Rc<RefCell<AssignedRoleGroup>>>,
    pub user_groups: Vec<Rc<RefCell<AssignedUserGroup>>>,
    pub departments: Vec<Rc<RefCell<AssignedDepartment>>>,
}

impl UserPermissions {
    pub fn flatten_permissions(&self) -> Vec<Permission> {
        todo!()
    }
}

impl AuthService {
    /// Query user permissions(explicit and implicit) by user id.
    pub async fn query_user_permissions(&self, user_id: Uuid) -> AppResult<UserPermissions> {
        let permission_service = PermissionService::new(self.0.clone());
        let role_service = RoleService::new(self.0.clone());
        let user_group_service = UserGroupService::new(self.0.clone());
        let department_service = DepartmentService::new(self.0.clone());
        let role_group_service = RoleGroupService::new(self.0.clone());
        let permission_group_service = PermissionGroupService::new(self.0.clone());

        let mut user_permissions = UserPermissions {
            departments: vec![],
            user_groups: vec![],
            permissions: vec![],
            permission_groups: vec![],
            roles: vec![],
            role_groups: vec![],
        };

        let mut permissions_map = HashMap::<Uuid, Rc<RefCell<Permission>>>::new();
        let mut permission_groups_map =
            HashMap::<Uuid, Rc<RefCell<AssignedPermissionGroup>>>::new();
        let mut roles_map = HashMap::<Uuid, Rc<RefCell<AssignedRole>>>::new();
        let mut role_groups_map = HashMap::<Uuid, Rc<RefCell<AssignedRoleGroup>>>::new();
        let mut user_groups_map = HashMap::<Uuid, Rc<RefCell<AssignedUserGroup>>>::new();
        let mut departments_map = HashMap::<Uuid, Rc<RefCell<AssignedDepartment>>>::new();

        // 1. query user departments
        let departments = department_service
            .query_departments_by_user_id(user_id)
            .await?
            .into_iter()
            .map(|x| {
                Rc::new(RefCell::new(AssignedDepartment {
                    department: x,
                    permissions: vec![],
                    permission_groups: vec![],
                    roles: vec![],
                    role_groups: vec![],
                }))
            })
            .collect::<Vec<_>>();
        for department in departments.iter() {
            departments_map.insert(department.borrow().department.id, department.clone());
        }

        // 2.1 query user_groups
        let user_groups = user_group_service
            .query_user_groups_by_user_id(user_id)
            .await?;
        let user_group_id_list = user_groups.iter().map(|group| group.id).collect::<Vec<_>>();
        for user_group in user_groups {
            let user_group_id = user_group.id;
            let user_group = Rc::new(RefCell::new(AssignedUserGroup {
                user_group,
                inherited_group: None,
                permissions: vec![],
                permission_groups: vec![],
                roles: vec![],
                role_groups: vec![],
            }));
            user_groups_map.insert(user_group_id, user_group.clone());
            user_permissions.user_groups.push(user_group.clone());
        }
        // query all user group ancestors
        let user_group_ancestors = user_group_service
            .query_many_user_group_ancestors(user_group_id_list)
            .await?;
        let mut user_group_ancestors = user_group_ancestors
            .into_iter()
            .map(|x| {
                Rc::new(RefCell::new(AssignedUserGroup {
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
            let id = user_group.borrow().user_group.id;
            user_groups_map.entry(id).or_insert(user_group.clone());
        }
        // assign user_group's inherited_group
        for user_group in user_group_ancestors.iter_mut() {
            let id = user_group.borrow().user_group.id;
            let parent_id = user_group.borrow().user_group.parent_id;
            if let Some(parent_id) = parent_id {
                let parent_group = user_groups_map.get(&parent_id).unwrap();
                let user_group = user_groups_map.get(&id).unwrap();
                user_group.borrow_mut().inherited_group = Some(parent_group.clone());
            }
        }

        // 3.1 query user role_groups
        let user_role_groups = role_group_service
            .query_role_groups_by_user_id(user_id)
            .await?
            .into_iter()
            .map(|x| {
                Rc::new(RefCell::new(AssignedRoleGroup {
                    role_group: x,
                    roles: vec![],
                    children: HashMap::new(),
                }))
            })
            .collect::<Vec<_>>();

        user_role_groups.iter().for_each(|x| {
            let id = x.borrow().role_group.id;
            role_groups_map.entry(id).or_insert(x.clone());
        });
        user_permissions.role_groups = user_role_groups
            .iter()
            .map(|x| {
                let id = x.borrow().role_group.id;
                role_groups_map.get(&id).unwrap().clone()
            })
            .collect();

        // 3.2 query department role_groups
        let department_id_list = departments_map
            .values()
            .map(|x| x.borrow().department.id)
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
                        .or_insert(Rc::new(RefCell::new(AssignedRoleGroup {
                            role_group,
                            roles: vec![],
                            children: HashMap::new(),
                        })));
                });
                // fill department
                departments_map
                    .get_mut(&dept_id)
                    .unwrap()
                    .borrow_mut()
                    .role_groups = role_groups_id_list
                    .into_iter()
                    .map(|role_group_id| role_groups_map.get(&role_group_id).unwrap().clone())
                    .collect();
            });

        // 3.3 query user_group role_groups
        let user_group_id_list = user_groups_map
            .values()
            .map(|x| x.borrow().user_group.id)
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
                        .or_insert(Rc::new(RefCell::new(AssignedRoleGroup {
                            role_group,
                            roles: vec![],
                            children: HashMap::new(),
                        })));
                });
                // fill department
                user_groups_map
                    .get_mut(&user_group_id)
                    .unwrap()
                    .borrow_mut()
                    .role_groups = role_groups_id_list
                    .into_iter()
                    .map(|role_group_id| role_groups_map.get(&role_group_id).unwrap().clone())
                    .collect();
            });
        // 3.4 query role_groups sub role_groups
        let role_group_id_list = role_groups_map.keys().cloned().collect::<Vec<_>>();
        let role_groups = role_group_service.query_role_groups_by_ancestors(role_group_id_list).await?;
        role_groups.into_iter().for_each(|role_group| {
            role_groups_map
                .entry(role_group.id)
                .or_insert(Rc::new(RefCell::new(AssignedRoleGroup {
                    role_group,
                    roles: vec![],
                    children: HashMap::new(),
                })));
        });
        // fill role_group children
        let mut parent_child_pairs = Vec::new();
        for role_group in role_groups_map.values() {
            let id = role_group.borrow().role_group.id;
            let parent_id = role_group.borrow().role_group.parent_id;
            if let Some(parent_id) = parent_id {
                parent_child_pairs.push((id, parent_id));
            }
        }
        for (id, parent_id) in parent_child_pairs {
            let child = role_groups_map.get(&id).unwrap().clone();
            let parent = role_groups_map.get_mut(&parent_id).unwrap();
            parent.borrow_mut().children.insert(id, child);
        }

        // 4.1 query user roles
        let roles = role_service.query_roles_by_user_id(user_id).await?;
        let role_id_list = roles.iter().map(|role| role.id).collect::<Vec<_>>();
        roles.into_iter().for_each(|role| {
            roles_map
                .entry(role.id)
                .or_insert(Rc::new(RefCell::new(AssignedRole {
                    role,
                    permissions: vec![],
                    permission_groups: vec![],
                })));
        });
        // assign user_permissions.roles
        user_permissions.roles = role_id_list
            .into_iter()
            .map(|role_id| roles_map.get(&role_id).unwrap().clone())
            .collect();

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
                    .or_insert(Rc::new(RefCell::new(AssignedRole {
                        role,
                        permissions: vec![],
                        permission_groups: vec![],
                    })));
            });
            // fill roles
            departments_map
                .get_mut(&department_id)
                .unwrap()
                .borrow_mut()
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
                    .or_insert(Rc::new(RefCell::new(AssignedRole {
                        role,
                        permissions: vec![],
                        permission_groups: vec![],
                    })));
            });
            // fill roles
            user_groups_map
                .get_mut(&user_group_id)
                .unwrap()
                .borrow_mut()
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
                    .or_insert(Rc::new(RefCell::new(AssignedRole {
                        role,
                        permissions: vec![],
                        permission_groups: vec![],
                    })));
            });
            // fill roles
            role_groups_map
                .get_mut(&role_group_id)
                .unwrap()
                .borrow_mut()
                .roles = role_id_list
                .into_iter()
                .map(|role_id| roles_map.get(&role_id).unwrap().clone())
                .collect();
        });

        // 5.1 query user permission_groups
        //
        // 5.2 query department permission_groups
        //
        // 5.3 query user_group permission_groups
        //
        // 5.4 query role permission_groups
        //
        // 5.5 query permission_group sub permission_groups

        // 6.1 query user permissions
        //
        // 6.2 query department permissions
        //
        // 6.3 query user_group permissions
        //
        // 6.4 query role permissions
        //
        // 6.5 query permission_group permissions

        Ok(user_permissions)
    }

    /// Query role permissions(explicit and implicit) by role id.
    pub async fn query_role_permissions(&self, role_id: Uuid) -> AppResult<()> {
        todo!()
    }

    /// Query department permissions(explicit and implicit) by department id.
    pub async fn query_department_permissions(&self, department_id: Uuid) -> AppResult<()> {
        todo!()
    }

    /// Query group permissions(explicit and implicit) by group id.
    pub async fn query_group_permissions(&self, group_id: Uuid) -> AppResult<()> {
        todo!()
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
