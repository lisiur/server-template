use sea_orm::prelude::*;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

use entity::{
    relation_role_groups_departments, relation_role_groups_user_groups, relation_role_groups_users,
    role_groups,
};
use uuid::Uuid;

use crate::{
    error::AppException,
    models::role_group::RoleGroup,
    result::AppResult,
    services::role_group::RoleGroupService,
    utils::query::{Cursor, FilterAtom, FilterCondition, PageableQuery, SelectQuery, TreeQuery},
};

impl RoleGroupService {
    pub async fn query_role_groups_by_page<T: PageableQuery<FilterRoleGroupsParams>>(
        &self,
        params: T,
    ) -> AppResult<(Vec<RoleGroup>, i64)> {
        let mut select_query = SelectQuery::default().with_cursor(params.cursor());
        let filter = params.into_filter();
        if let Some(ref name) = filter.name {
            if !name.is_empty() {
                select_query.add_atom_filter(FilterAtom {
                    field: role_groups::Column::Name.as_str().to_string(),
                    condition: FilterCondition::Like(format!("%{name}%")),
                });
            }
        }
        let (role_groups, count) = select_query
            .all_with_count::<role_groups::Model>(role_groups::Entity, &self.0)
            .await?;
        let role_groups = role_groups.into_iter().map(RoleGroup::from).collect();

        Ok((role_groups, count))
    }

    pub async fn query_role_group_tree(&self, role_group_id: Uuid) -> AppResult<RoleGroupTree> {
        let role_groups = TreeQuery::new(role_groups::Entity)
            .query_descendants_with_one(&self.0, role_group_id)
            .await?
            .into_iter()
            .map(|x| {
                Rc::new(RefCell::new(RoleGroupTreeNode {
                    role_group: x.into(),
                    children: vec![],
                }))
            })
            .collect::<Vec<_>>();
        if role_groups.is_empty() {
            return Err(AppException::RoleGroupNotFound.into());
        }

        let mut role_group_tree_nodes: HashMap<Uuid, Rc<RefCell<RoleGroupTreeNode>>> =
            HashMap::new();

        // record all role_groups
        for role_group in &role_groups {
            role_group_tree_nodes.insert(role_group.borrow().role_group.id, role_group.clone());
        }

        // fill role_group children
        for role_group in &role_groups {
            let parent_id = role_group.borrow().role_group.parent_id;
            if let Some(parent_id) = parent_id {
                let parent_role_group = role_group_tree_nodes.get(&parent_id).unwrap();
                parent_role_group
                    .borrow_mut()
                    .children
                    .push(role_group.clone());
            }
        }

        let root_node = role_group_tree_nodes.get(&role_group_id).unwrap().clone();

        let tree = RoleGroupTree(root_node);

        Ok(tree)
    }

    pub async fn query_role_groups_by_ancestors(
        &self,
        role_group_id_list: Vec<Uuid>,
    ) -> AppResult<Vec<RoleGroup>> {
        let role_groups = TreeQuery::new(role_groups::Entity)
            .query_descendants_with_many(&self.0, role_group_id_list)
            .await?;

        Ok(role_groups.into_iter().map(RoleGroup::from).collect())
    }

    pub async fn query_role_groups_by_user_id(&self, user_id: Uuid) -> AppResult<Vec<RoleGroup>> {
        let groups = role_groups::Entity::find()
            .inner_join(relation_role_groups_users::Entity)
            .filter(relation_role_groups_users::Column::UserId.eq(user_id))
            .all(&self.0)
            .await?;

        Ok(groups.into_iter().map(RoleGroup::from).collect())
    }

    pub async fn query_role_groups_by_department_id(
        &self,
        department_id: Uuid,
    ) -> AppResult<Vec<RoleGroup>> {
        let groups = role_groups::Entity::find()
            .inner_join(relation_role_groups_departments::Entity)
            .filter(relation_role_groups_departments::Column::DepartmentId.eq(department_id))
            .all(&self.0)
            .await?;

        Ok(groups.into_iter().map(RoleGroup::from).collect())
    }

    pub async fn query_role_groups_by_department_id_list(
        &self,
        department_id_list: Vec<Uuid>,
    ) -> AppResult<HashMap<Uuid, Vec<RoleGroup>>> {
        let results = role_groups::Entity::find()
            .inner_join(relation_role_groups_departments::Entity)
            .find_also_related(relation_role_groups_departments::Entity)
            .filter(
                relation_role_groups_departments::Column::DepartmentId.is_in(department_id_list),
            )
            .all(&self.0)
            .await?;
        let mut map = HashMap::new();
        for (role_group, relation) in results {
            let department_id = relation.unwrap().department_id;
            map.entry(department_id)
                .or_insert_with(Vec::new)
                .push(RoleGroup::from(role_group));
        }
        Ok(map)
    }

    pub async fn query_role_groups_by_user_group_id_list(
        &self,
        user_group_id_list: Vec<Uuid>,
    ) -> AppResult<HashMap<Uuid, Vec<RoleGroup>>> {
        let results = role_groups::Entity::find()
            .inner_join(relation_role_groups_user_groups::Entity)
            .find_also_related(relation_role_groups_user_groups::Entity)
            .filter(relation_role_groups_user_groups::Column::UserGroupId.is_in(user_group_id_list))
            .all(&self.0)
            .await?;
        let mut map = HashMap::new();
        for (role_group, relation) in results {
            let user_group_id = relation.unwrap().user_group_id;
            map.entry(user_group_id)
                .or_insert_with(Vec::new)
                .push(RoleGroup::from(role_group));
        }
        Ok(map)
    }

    pub async fn query_role_groups_by_user_group_id(
        &self,
        user_group_id: Uuid,
    ) -> AppResult<Vec<RoleGroup>> {
        let groups = role_groups::Entity::find()
            .inner_join(relation_role_groups_user_groups::Entity)
            .filter(relation_role_groups_user_groups::Column::UserGroupId.eq(user_group_id))
            .all(&self.0)
            .await?;

        Ok(groups.into_iter().map(RoleGroup::from).collect())
    }
}

pub struct RoleGroupTree(pub Rc<RefCell<RoleGroupTreeNode>>);

pub struct RoleGroupTreeNode {
    pub role_group: RoleGroup,
    pub children: Vec<Rc<RefCell<RoleGroupTreeNode>>>,
}

pub struct QueryRoleGroupsByPageParams {
    pub cursor: Cursor,
    pub name: Option<String>,
}

pub struct FilterRoleGroupsParams {
    pub name: Option<String>,
}
