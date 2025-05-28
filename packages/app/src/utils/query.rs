use migration::{Alias, ExprTrait, IntoColumnRef, IntoCondition, IntoTableRef, SelectStatement};
use sea_orm::{ConnectionTrait, DatabaseConnection, FromQueryResult, sea_query};
use serde::Deserialize;
use serde_json::Value;
use utoipa::ToSchema;

use crate::result::AppResult;

#[derive(Clone, Deserialize)]
pub struct SelectQuery {
    pub filters: Option<Filter>,
    pub orders: Option<Vec<Order>>,
    pub pagination: Option<Pagination>,
}

impl SelectQuery {
    pub fn into_select_statement(self, table: impl IntoTableRef) -> SelectStatement {
        let SelectQuery {
            filters,
            orders,
            pagination,
        } = self;

        let mut select_query = sea_query::Query::select();
        select_query.from(table.into_table_ref());
        if let Some(filters) = filters {
            select_query.cond_where(filters);
        }
        if let Some(orders) = orders {
            for order in orders {
                match order.direction {
                    OrderDirection::Asc => {
                        select_query.order_by(Alias::new(&order.field), sea_query::Order::Asc);
                    }
                    OrderDirection::Desc => {
                        select_query.order_by(Alias::new(&order.field), sea_query::Order::Desc);
                    }
                }
            }
        }
        if let Some(pagination) = pagination {
            select_query.offset(pagination.offset);
            select_query.limit(pagination.limit);
        }

        select_query
    }

    pub async fn all<T: FromQueryResult>(
        self,
        db: &DatabaseConnection,
        table: impl IntoTableRef,
    ) -> AppResult<Vec<T>> {
        let stmt = db
            .get_database_backend()
            .build(&self.into_select_statement(table));
        let result = T::find_by_statement(stmt).all(db).await?;

        Ok(result)
    }

    pub async fn one<T: FromQueryResult>(
        self,
        db: &DatabaseConnection,
        table: impl IntoTableRef,
    ) -> AppResult<Option<T>> {
        let stmt = db
            .get_database_backend()
            .build(&self.into_select_statement(table));
        let result = T::find_by_statement(stmt).one(db).await?;

        Ok(result)
    }

    pub async fn all_with_count<T: FromQueryResult>(
        self,
        db: &DatabaseConnection,
        table: impl IntoTableRef,
    ) -> AppResult<(Vec<T>, u64)> {
        let table_ref = table.into_table_ref();
        let count = if let Some(filter) = &self.filters {
            filter.clone().count(db, table_ref.clone()).await?
        } else {
            let mut select_query = sea_query::Query::select();
            select_query.from(table_ref.clone());

            let builder = db.get_database_backend();
            select_query.expr_as(
                sea_query::Expr::col(sea_query::Asterisk).count(),
                Alias::new("count"),
            );
            let stmt = builder.build(&select_query);
            let res = db.query_one(stmt).await?.unwrap();
            let count = res.try_get::<u64>("", "count")?;
            count
        };
        let stmt = db
            .get_database_backend()
            .build(&self.into_select_statement(table_ref));
        let result = T::find_by_statement(stmt).all(db).await?;

        Ok((result, count))
    }
}

#[derive(Clone, Deserialize)]
pub struct Pagination {
    pub limit: u64,
    pub offset: u64,
}

#[derive(Clone, Deserialize)]
#[serde(tag = "type", content = "")]
pub enum Filter {
    Atom(FilterAtom),
    And(Vec<Filter>),
    Or(Vec<Filter>),
}

#[derive(Clone, Deserialize)]
pub struct FilterAtom {
    pub field: String,
    #[serde(flatten)]
    pub condition: FilterCondition,
}

#[derive(Clone, Deserialize, ToSchema)]
#[serde(tag = "operator", content = "value")]
pub enum FilterCondition {
    Equals(Value),
    Gt(Value),
    Ge(Value),
    Lt(Value),
    Le(Value),
    In(Vec<Value>),
    NotIn(Vec<Value>),
    Like(String),
    NotLike(String),
    Between(Vec<Value>),
    IsNull,
    IsNotNull,
}

#[derive(Clone, Deserialize, ToSchema)]
pub struct Order {
    pub field: String,
    pub direction: OrderDirection,
}

#[derive(Clone, Deserialize, ToSchema)]
pub enum OrderDirection {
    Asc,
    Desc,
}

impl Filter {
    pub async fn count(self, db: &DatabaseConnection, table: impl IntoTableRef) -> AppResult<u64> {
        let mut select_query = sea_query::Query::select();
        select_query.from(table.into_table_ref());
        select_query.cond_where(self);

        let builder = db.get_database_backend();
        select_query.expr_as(
            sea_query::Expr::col(sea_query::Asterisk).count(),
            Alias::new("count"),
        );
        let stmt = builder.build(&select_query);
        let res = db.query_one(stmt).await?.unwrap();
        let count = res.try_get::<u64>("", "count")?;

        Ok(count)
    }
}

impl IntoCondition for Filter {
    fn into_condition(self) -> sea_orm::Condition {
        handle_filter(self)
    }
}

fn handle_filter(filter: Filter) -> sea_orm::Condition {
    match filter {
        Filter::Atom(atom) => handle_filter_atom(atom),
        Filter::And(filters) => {
            let mut conditions = sea_orm::Condition::all();
            for filter in filters {
                conditions = conditions.add(handle_filter(filter).into_condition());
            }
            conditions
        }
        Filter::Or(filters) => {
            let mut conditions = sea_orm::Condition::any();
            for filter in filters {
                conditions = conditions.add(handle_filter(filter).into_condition());
            }
            conditions
        }
    }
}

fn handle_filter_atom(filter_atom: FilterAtom) -> sea_orm::Condition {
    let FilterAtom { field, condition } = filter_atom;
    let field = Alias::new(field).into_column_ref();
    match condition {
        FilterCondition::Equals(value) => field.eq(value).into_condition(),
        FilterCondition::Gt(value) => field.gt(value).into_condition(),
        FilterCondition::Ge(value) => field.gte(value).into_condition(),
        FilterCondition::Lt(value) => field.lt(value).into_condition(),
        FilterCondition::Le(value) => field.lte(value).into_condition(),
        FilterCondition::In(value) => field.is_in(value).into_condition(),
        FilterCondition::NotIn(value) => field.is_not_in(value).into_condition(),
        FilterCondition::Like(value) => field.like(value).into_condition(),
        FilterCondition::NotLike(value) => field.not_like(value).into_condition(),
        FilterCondition::Between(value) => field
            .between(value[0].clone(), value[1].clone())
            .into_condition(),
        FilterCondition::IsNull => field.is_null().into_condition(),
        FilterCondition::IsNotNull => field.is_not_null().into_condition(),
    }
}
