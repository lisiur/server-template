use app::utils::query::{DisableOrder, Order, PageableQuery};
use sea_orm::sea_query::{Alias, ExprTrait, IntoColumnRef, IntoCondition};
use serde::Deserialize;
use serde_json::Value;
use utoipa::{IntoParams, ToSchema};

#[derive(Deserialize, IntoParams)]
#[serde(rename_all = "camelCase")]
#[into_params(parameter_in = Query)]
#[allow(dead_code)]
pub struct PaginatedQueryParams {
    #[param(minimum = 1)]
    pub page: u64,
    #[param(minimum = 0)]
    pub page_size: u64,
}

#[derive(Clone, Deserialize, ToSchema)]
pub struct OrderDto<T> {
    field: T,
    order: Order,
}

#[derive(Clone, Deserialize, ToSchema)]
#[serde(rename_all = "camelCase")]
pub struct PageableQueryDto<T, U = DisableOrder> {
    pub page: u64,
    pub page_size: u64,
    pub filter: Option<T>,
    pub orders: Option<Vec<OrderDto<U>>>,
}

impl<T, U, P: From<T>, Q: From<U>> From<PageableQueryDto<T, U>> for PageableQuery<P, Q> {
    fn from(value: PageableQueryDto<T, U>) -> Self {
        PageableQuery {
            page: value.page,
            page_size: value.page_size,
            orders: value.orders.map(|value| {
                value
                    .into_iter()
                    .map(
                        |OrderDto {
                             field: column,
                             order,
                         }| (column.into(), order),
                    )
                    .collect()
            }),
            condition: value.filter.map(Into::into),
        }
    }
}

#[derive(Clone, Deserialize)]
#[serde(tag = "type", content = "")]
pub enum Filter {
    Atom(AtomFilter),
    And(Vec<Filter>),
    Or(Vec<Filter>),
}

#[derive(Clone, Deserialize)]
pub struct AtomFilter {
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

impl IntoCondition for Filter {
    fn into_condition(self) -> sea_orm::Condition {
        handle_filter(self)
    }
}

fn handle_filter(filter: Filter) -> sea_orm::Condition {
    match filter {
        Filter::Atom(atom) => handle_atom_filter(atom),
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

fn handle_atom_filter(filter_atom: AtomFilter) -> sea_orm::Condition {
    let AtomFilter { field, condition } = filter_atom;
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
