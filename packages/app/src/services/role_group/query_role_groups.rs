use sea_orm::{Condition, prelude::*};
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
    utils::query::{Cursor, PageableQuery, TreeQuery},
};

pub struct FilterRoleGroupsParams {
    pub name: Option<String>,
}

impl From<FilterRoleGroupsParams> for Condition {
    fn from(value: FilterRoleGroupsParams) -> Self {
        Condition::all().add_option(value.name.map(|name| role_groups::Column::Name.like(name)))
    }
}

impl RoleGroupService {
    pub async fn query_role_groups_by_page(
        &self,
        params: PageableQuery<FilterRoleGroupsParams>,
    ) -> AppResult<(Vec<RoleGroup>, i64)> {
        let (records, total) = self.crud.find_by_condition_with_count(params).await?;
        let role_groups = records.into_iter().map(RoleGroup::from).collect();

        Ok((role_groups, total))
    }

    pub async fn query_role_group_by_id(&self, role_group_id: Uuid) -> AppResult<RoleGroup> {
        let role_group = role_groups::Entity::find_by_id(role_group_id)
            .one(&self.conn)
            .await?
            .ok_or(AppException::RoleGroupNotFound)?;

        Ok(RoleGroup::from(role_group))
    }

    pub async fn query_role_group_tree(&self, role_group_id: Uuid) -> AppResult<RoleGroupTree> {
        let role_groups = TreeQuery::new(role_groups::Entity)
            .query_descendants_with_one(&self.conn, role_group_id)
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
            .query_descendants_with_many(&self.conn, role_group_id_list)
            .await?;

        Ok(role_groups.into_iter().map(RoleGroup::from).collect())
    }

    pub async fn query_role_groups_by_user_id(&self, user_id: Uuid) -> AppResult<Vec<RoleGroup>> {
        let groups = role_groups::Entity::find()
            .inner_join(relation_role_groups_users::Entity)
            .filter(relation_role_groups_users::Column::UserId.eq(user_id))
            .all(&self.conn)
            .await?;

        Ok(groups.into_iter().map(RoleGroup::from).collect())
    }

    pub async fn query_role_groups_by_user_id_list(
        &self,
        user_id_list: Vec<Uuid>,
    ) -> AppResult<HashMap<Uuid, Vec<RoleGroup>>> {
        if user_id_list.is_empty() {
            return Ok(HashMap::new());
        }

        let results = role_groups::Entity::find()
            .find_also_related(relation_role_groups_users::Entity)
            .filter(relation_role_groups_users::Column::UserId.is_in(user_id_list))
            .all(&self.conn)
            .await?;

        let mut map = HashMap::new();
        for (role_group, relation) in results {
            let user_id = relation.unwrap().user_id;
            map.entry(user_id)
                .or_insert_with(Vec::new)
                .push(RoleGroup::from(role_group));
        }

        Ok(map)
    }

    pub async fn query_role_groups_by_department_id(
        &self,
        department_id: Uuid,
    ) -> AppResult<Vec<RoleGroup>> {
        let groups = role_groups::Entity::find()
            .inner_join(relation_role_groups_departments::Entity)
            .filter(relation_role_groups_departments::Column::DepartmentId.eq(department_id))
            .all(&self.conn)
            .await?;

        Ok(groups.into_iter().map(RoleGroup::from).collect())
    }

    pub async fn query_role_groups_by_department_id_list(
        &self,
        department_id_list: Vec<Uuid>,
    ) -> AppResult<HashMap<Uuid, Vec<RoleGroup>>> {
        let results = role_groups::Entity::find()
            .find_also_related(relation_role_groups_departments::Entity)
            .filter(
                relation_role_groups_departments::Column::DepartmentId.is_in(department_id_list),
            )
            .all(&self.conn)
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
            .find_also_related(relation_role_groups_user_groups::Entity)
            .filter(relation_role_groups_user_groups::Column::UserGroupId.is_in(user_group_id_list))
            .all(&self.conn)
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
            .all(&self.conn)
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
