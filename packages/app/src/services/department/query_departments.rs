use sea_orm::{Statement, prelude::*};
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
    utils::query::{Cursor, FilterAtom, FilterCondition, PageableQuery, SelectQuery},
};

impl DepartmentService {
    pub async fn query_departments_by_page<T: PageableQuery<FilterDepartmentsParams>>(
        &self,
        params: T,
    ) -> AppResult<(Vec<Department>, i64)> {
        let mut select_query = SelectQuery::default().with_cursor(params.cursor());
        let filter = params.into_filter();
        if let Some(ref name) = filter.name {
            if !name.is_empty() {
                select_query.add_atom_filter(FilterAtom {
                    field: departments::Column::Name.as_str().to_string(),
                    condition: FilterCondition::Like(format!("%{name}%")),
                });
            }
        }
        let (departments, count) = select_query
            .all_with_count::<departments::Model>(departments::Entity, &self.0)
            .await?;
        let departments = departments.into_iter().map(Department::from).collect();

        Ok((departments, count))
    }

    pub async fn query_department_tree(&self, department_id: Uuid) -> AppResult<DepartmentTree> {
        let departments = self.query_department_tree_models(department_id).await?;
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

    pub async fn query_department_chain(&self, department_id: Uuid) -> AppResult<Vec<Department>> {
        let departments = self.query_department_chain_models(department_id).await?;
        if departments.is_empty() {
            return Err(AppException::UserGroupNotFound.into());
        }

        Ok(departments.into_iter().map(Department::from).collect())
    }

    pub async fn query_department_tree_models(
        &self,
        department_id: Uuid,
    ) -> AppResult<Vec<departments::Model>> {
        let departments = departments::Entity::find()
            .from_raw_sql(Statement::from_sql_and_values(
                self.0.get_database_backend(),
                format!(
                    r#"
                        WITH RECURSIVE group_tree AS (
                            SELECT * FROM {table} WHERE {id} = $1
                            UNION ALL
                            SELECT g.* FROM {table} g
                            JOIN group_tree gt ON g.{parent_id} = gt.{id}
                        )
                        SELECT * FROM group_tree
                    "#,
                    table = entity::departments::Entity.as_str(),
                    id = entity::departments::Column::Id.as_str(),
                    parent_id = entity::departments::Column::ParentId.as_str(),
                ),
                vec![department_id.into()],
            ))
            .all(&self.0)
            .await?;
        Ok(departments)
    }

    pub async fn query_department_chain_models(
        &self,
        department_id: Uuid,
    ) -> AppResult<Vec<departments::Model>> {
        let departments = departments::Entity::find()
            .from_raw_sql(Statement::from_sql_and_values(
                self.0.get_database_backend(),
                format!(
                    r#"
                        WITH RECURSIVE group_chain AS (
                            SELECT * FROM {table} WHERE {id} = $1
                            UNION ALL
                            SELECT g.* FROM {table} g
                            JOIN group_chain gc ON g.{id} = gc.{parent_id}
                        )
                        SELECT * FROM group_chain
                    "#,
                    table = entity::departments::Entity.as_str(),
                    id = entity::departments::Column::Id.as_str(),
                    parent_id = entity::departments::Column::ParentId.as_str(),
                ),
                vec![department_id.into()],
            ))
            .all(&self.0)
            .await?;

        Ok(departments)
    }

    pub async fn query_departments_by_user_id(&self, user_id: Uuid) -> AppResult<Vec<Department>> {
        let groups = departments::Entity::find()
            .inner_join(relation_users_departments::Entity)
            .filter(relation_users_departments::Column::UserId.eq(user_id))
            .all(&self.0)
            .await?;

        Ok(groups.into_iter().map(Department::from).collect())
    }

    pub async fn query_departments_by_role_id(&self, role_id: Uuid) -> AppResult<Vec<Department>> {
        let departments = departments::Entity::find()
            .inner_join(relation_roles_departments::Entity)
            .filter(relation_roles_departments::Column::RoleId.eq(role_id))
            .all(&self.0)
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

pub struct FilterDepartmentsParams {
    pub name: Option<String>,
}
