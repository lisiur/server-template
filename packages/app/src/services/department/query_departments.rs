use sea_orm::{Condition, prelude::*};
use serde::Serialize;
use std::{cell::RefCell, collections::HashMap, rc::Rc};
use utoipa::ToSchema;

use entity::{departments, relation_roles_departments, relation_users_departments};
use uuid::Uuid;

use crate::{
    error::AppException,
    models::department::Department,
    result::AppResult,
    services::department::DepartmentService,
    utils::query::{Cursor, PageableQuery, TreeQuery},
};

pub struct FilterDepartmentsParams {
    pub name: Option<String>,
}

impl From<FilterDepartmentsParams> for Condition {
    fn from(value: FilterDepartmentsParams) -> Self {
        Condition::all().add_option(value.name.map(|name| departments::Column::Name.like(name)))
    }
}

impl DepartmentService {
    pub async fn query_departments_by_page(
        &self,
        params: PageableQuery<FilterDepartmentsParams>,
    ) -> AppResult<(Vec<Department>, i64)> {
        let (departments, count) = self.crud.find_by_condition_with_count(params).await?;
        let departments = departments.into_iter().map(Department::from).collect();

        Ok((departments, count))
    }

    pub async fn query_department_by_id(&self, department_id: Uuid) -> AppResult<Department> {
        let department = departments::Entity::find_by_id(department_id)
            .one(&self.conn)
            .await?;

        let Some(department) = department else {
            return Err(AppException::DepartmentNotFound.into());
        };

        Ok(Department::from(department))
    }

    pub async fn query_department_tree(&self, department_id: Uuid) -> AppResult<DepartmentTree> {
        let departments = TreeQuery::new(departments::Entity)
            .query_descendants_with_one(&self.conn, department_id)
            .await?;

        if departments.is_empty() {
            return Err(AppException::UserGroupNotFound.into());
        }

        let mut department_nodes: HashMap<Uuid, Rc<RefCell<DepartmentTreeNode>>> = HashMap::new();

        for department in &departments {
            let node = Rc::new(RefCell::new(DepartmentTreeNode {
                id: department.id,
                name: department.name.clone(),
                description: department.description.clone(),
                children: vec![],
            }));
            department_nodes.insert(department.id, node);
        }

        let root_node = department_nodes.get(&department_id).unwrap().clone();

        let tree = DepartmentTree(root_node);

        Ok(tree)
    }

    pub async fn query_department_ancestors(
        &self,
        department_id: Uuid,
    ) -> AppResult<Vec<Department>> {
        let departments = TreeQuery::new(departments::Entity)
            .query_ancestors_with_one(&self.conn, department_id)
            .await?;

        if departments.is_empty() {
            return Err(AppException::UserGroupNotFound.into());
        }

        Ok(departments.into_iter().map(Department::from).collect())
    }

    pub async fn query_departments_by_user_id(&self, user_id: Uuid) -> AppResult<Vec<Department>> {
        let groups = departments::Entity::find()
            .inner_join(relation_users_departments::Entity)
            .filter(relation_users_departments::Column::UserId.eq(user_id))
            .all(&self.conn)
            .await?;

        Ok(groups.into_iter().map(Department::from).collect())
    }

    pub async fn query_departments_by_user_id_list(
        &self,
        user_id_list: Vec<Uuid>,
    ) -> AppResult<HashMap<Uuid, Vec<Department>>> {
        if user_id_list.is_empty() {
            return Ok(HashMap::new());
        }

        let results = departments::Entity::find()
            .find_also_related(relation_users_departments::Entity)
            .filter(relation_users_departments::Column::UserId.is_in(user_id_list))
            .all(&self.conn)
            .await?;

        let mut map = HashMap::new();
        for (department, relation) in results {
            let user_id = relation.unwrap().user_id;
            map.entry(user_id)
                .or_insert_with(Vec::new)
                .push(Department::from(department));
        }

        Ok(map)
    }

    pub async fn query_departments_by_role_id(&self, role_id: Uuid) -> AppResult<Vec<Department>> {
        let departments = departments::Entity::find()
            .inner_join(relation_roles_departments::Entity)
            .filter(relation_roles_departments::Column::RoleId.eq(role_id))
            .all(&self.conn)
            .await?;

        Ok(departments.into_iter().map(Department::from).collect())
    }
}

pub struct DepartmentTree(pub Rc<RefCell<DepartmentTreeNode>>);

#[derive(ToSchema, Serialize)]
pub struct DepartmentTreeNode {
    pub id: Uuid,
    pub name: String,
    pub description: Option<String>,
    #[schema(no_recursion)]
    pub children: Vec<Rc<RefCell<DepartmentTreeNode>>>,
}

pub struct QueryDepartmentsByPageParams {
    pub cursor: Cursor,
    pub name: Option<String>,
}
