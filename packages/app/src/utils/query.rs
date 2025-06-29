use migration::{ColumnRef, ConditionExpression};
use sea_orm::{Condition, ConnectionTrait, DatabaseConnection, EntityTrait, Statement};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::result::AppResult;

#[derive(Clone)]
pub struct Sort {
    pub column_ref: ColumnRef,
    pub order: Order,
}

#[derive(Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
#[schema(rename_all = "camelCase")]
pub enum Order {
    Asc,
    Desc,
}

impl From<Order> for sea_orm::Order {
    fn from(value: Order) -> Self {
        match value {
            Order::Asc => Self::Asc,
            Order::Desc => Self::Desc,
        }
    }
}

#[derive(Clone, Default)]
pub struct QueryCondition {
    pub condition: Option<Condition>,
    pub orders: Option<Vec<Sort>>,
    pub cursor: Option<Cursor>,
}

impl<T: Into<ConditionExpression>> From<T> for QueryCondition {
    fn from(value: T) -> Self {
        Self::default().with_condition(Condition::all().add(value.into()))
    }
}

impl QueryCondition {
    pub fn with_condition(mut self, condition: Condition) -> Self {
        self.condition = Some(condition);
        self
    }

    pub fn with_cursor(mut self, cursor: Cursor) -> Self {
        self.cursor = Some(cursor);
        self
    }

    pub fn with_orders(mut self, orders: Vec<Sort>) -> Self {
        self.orders = Some(orders);
        self
    }

    pub fn clear_cursor(mut self) -> Self {
        self.cursor = None;
        self
    }

    pub fn clear_orders(mut self) -> Self {
        self.orders = None;
        self
    }
}

#[derive(Clone, Deserialize)]
pub struct Cursor {
    pub limit: u64,
    pub offset: u64,
}

#[derive(Clone, Deserialize, ToSchema)]
pub struct DisableOrder;

pub enum OrderField {
    ColumnRef(ColumnRef),
    Unspecified,
}

pub struct PageableQuery<T, U = DisableOrder> {
    pub page: u64,
    pub page_size: u64,
    pub condition: Option<T>,
    pub orders: Option<Vec<(U, Order)>>,
}

impl<T: Into<Condition>> From<PageableQuery<T>> for QueryCondition {
    fn from(value: PageableQuery<T>) -> Self {
        Self {
            condition: value.condition.map(Into::into),
            orders: None,
            cursor: Some(Cursor {
                limit: value.page_size,
                offset: (value.page - 1) * value.page_size,
            }),
        }
    }
}

impl<T: Into<Condition>, U: Into<ColumnRef>> From<PageableQuery<T, U>> for QueryCondition {
    fn from(value: PageableQuery<T, U>) -> Self {
        Self {
            condition: value.condition.map(Into::into),
            orders: match value.orders {
                None => None,
                Some(value) => Some(
                    value
                        .into_iter()
                        .map(|(column, order)| Sort {
                            column_ref: column.into(),
                            order,
                        })
                        .collect(),
                ),
            },
            cursor: Some(Cursor {
                limit: value.page_size,
                offset: (value.page - 1) * value.page_size,
            }),
        }
    }
}

pub struct TreeQuery<T: EntityTrait>(T);

impl<T: EntityTrait> TreeQuery<T> {
    pub fn new(entity: T) -> Self {
        Self(entity)
    }

    pub async fn query_descendants(&self, db: &DatabaseConnection) -> AppResult<Vec<T::Model>> {
        let models = T::find()
            .from_raw_sql(Statement::from_sql_and_values(
                db.get_database_backend(),
                format!(
                    r#"
                        WITH RECURSIVE tree AS (
                            SELECT * FROM {table} WHERE {id} IS NULL
                            UNION ALL
                            SELECT g.* FROM {table} g
                            JOIN tree t ON g.{parent_id} = t.{id}
                        )
                        SELECT * FROM tree
                    "#,
                    table = self.0.as_str(),
                    id = "id",
                    parent_id = "parent_id",
                ),
                vec![],
            ))
            .all(db)
            .await?;

        Ok(models)
    }

    pub async fn query_descendants_with_one(
        &self,
        db: &DatabaseConnection,
        id: Uuid,
    ) -> AppResult<Vec<T::Model>> {
        let models = T::find()
            .from_raw_sql(Statement::from_sql_and_values(
                db.get_database_backend(),
                format!(
                    r#"
                        WITH RECURSIVE tree AS (
                            SELECT * FROM {table} WHERE {id} = $1
                            UNION ALL
                            SELECT g.* FROM {table} g
                            JOIN tree t ON g.{parent_id} = t.{id}
                        )
                        SELECT * FROM tree
                    "#,
                    table = self.0.as_str(),
                    id = "id",
                    parent_id = "parent_id",
                ),
                vec![id.into()],
            ))
            .all(db)
            .await?;

        Ok(models)
    }

    pub async fn query_descendants_with_many(
        &self,
        db: &DatabaseConnection,
        ids: Vec<Uuid>,
    ) -> AppResult<Vec<T::Model>> {
        if ids.is_empty() {
            return Ok(vec![]);
        }
        let models = T::find()
            .from_raw_sql(Statement::from_sql_and_values(
                db.get_database_backend(),
                format!(
                    r#"
                        WITH RECURSIVE tree AS (
                            SELECT * FROM {table} WHERE {id} = ANY($1)
                            UNION ALL
                            SELECT g.* FROM {table} g
                            JOIN tree t ON g.{parent_id} = t.{id}
                        )
                        SELECT * FROM tree
                    "#,
                    table = self.0.as_str(),
                    id = "id",
                    parent_id = "parent_id",
                ),
                vec![ids.into()],
            ))
            .all(db)
            .await?;

        Ok(models)
    }

    pub async fn query_ancestors_with_one(
        &self,
        db: &DatabaseConnection,
        id: Uuid,
    ) -> AppResult<Vec<T::Model>> {
        let models = T::find()
            .from_raw_sql(Statement::from_sql_and_values(
                db.get_database_backend(),
                format!(
                    r#"
                        WITH RECURSIVE tree AS (
                            SELECT * FROM {table} WHERE {id} = $1
                            UNION ALL
                            SELECT g.* FROM {table} g
                            JOIN tree t ON g.{id} = t.{parent_id}
                        )
                        SELECT * FROM tree
                    "#,
                    table = self.0.as_str(),
                    id = "id",
                    parent_id = "parent_id",
                ),
                vec![id.into()],
            ))
            .all(db)
            .await?;

        Ok(models)
    }

    pub async fn query_ancestors_with_many(
        &self,
        db: &DatabaseConnection,
        ids: Vec<Uuid>,
    ) -> AppResult<Vec<T::Model>> {
        if ids.len() == 0 {
            return Ok(vec![]);
        }
        let models = T::find()
            .from_raw_sql(Statement::from_sql_and_values(
                db.get_database_backend(),
                format!(
                    r#"
                        WITH RECURSIVE tree AS (
                            SELECT * FROM {table} WHERE {id} = ANY($1)
                            UNION ALL
                            SELECT g.* FROM {table} g
                            JOIN tree t ON g.{id} = t.{parent_id}
                        )
                        SELECT * FROM tree
                    "#,
                    table = self.0.as_str(),
                    id = "id",
                    parent_id = "parent_id",
                ),
                vec![ids.into()],
            ))
            .all(db)
            .await?;

        Ok(models)
    }
}
