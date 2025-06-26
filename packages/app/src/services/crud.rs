use migration::{Alias, ConditionExpression, IntoCondition, SimpleExpr};
use sea_orm::{
    Condition, IntoActiveModel, QueryOrder, QuerySelect, QueryTrait, prelude::*, sea_query,
};
use std::marker::PhantomData;

use crate::utils::query::{Cursor, QueryCondition};

pub struct Crud<T>
where
    T: EntityTrait + Send + Sync,
{
    db: DatabaseConnection,
    _marker: PhantomData<T>,
}

impl<T> Crud<T>
where
    T: EntityTrait + Send + Sync,
{
    pub fn new(db: DatabaseConnection) -> Self {
        Self {
            db,
            _marker: PhantomData,
        }
    }
}

impl<T> Crud<T>
where
    T: EntityTrait,
    <T as EntityTrait>::ActiveModel:
        ActiveModelTrait + Send + Sync + From<<T as EntityTrait>::Model>,
    <T as EntityTrait>::Model: IntoActiveModel<<T as EntityTrait>::ActiveModel>,
{
    pub async fn create(
        &self,
        model: impl Into<<T as EntityTrait>::ActiveModel> + Send + Sync,
    ) -> Result<<T as EntityTrait>::Model, DbErr> {
        let active_model: <T as EntityTrait>::ActiveModel = model.into();
        let res = active_model.insert(&self.db).await?;
        Ok(res)
    }

    pub async fn find_by_id(
        &self,
        id: impl Into<<<T as sea_orm::EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType>,
    ) -> Result<Option<<T as EntityTrait>::Model>, DbErr> {
        let res = T::find_by_id(id).one(&self.db).await?;
        Ok(res)
    }

    fn find_condition_to_select(condition: QueryCondition) -> Select<T> {
        let mut select_query = T::find();
        let QueryCondition {
            condition,
            orders,
            cursor,
        } = condition;
        if let Some(condition) = condition {
            select_query = select_query.filter(condition);
        }

        if let Some(orders) = orders {
            for order in orders {
                select_query =
                    select_query.order_by(SimpleExpr::Column(order.column_ref), order.order.into());
            }
        }

        if let Some(Cursor { limit, offset }) = cursor {
            select_query = select_query.limit(limit).offset(offset);
        }
        select_query
    }

    pub async fn find_by_condition_with_count(
        &self,
        condition: impl Into<QueryCondition>,
    ) -> Result<(Vec<<T as EntityTrait>::Model>, i64), DbErr> {
        let condition = condition.into();
        let count_condition: QueryCondition = condition.clone();
        let count = self.count_by_condition(count_condition).await?;
        let select_query = Self::find_condition_to_select(condition);
        let res = select_query.all(&self.db).await?;
        Ok((res, count))
    }

    pub async fn find_by_condition(
        &self,
        condition: impl Into<QueryCondition>,
    ) -> Result<Vec<<T as EntityTrait>::Model>, DbErr> {
        let select_query = Self::find_condition_to_select(condition.into());
        let res = select_query.all(&self.db).await?;
        Ok(res)
    }

    pub async fn find_one_by_condition(
        &self,
        condition: impl Into<ConditionExpression>,
    ) -> Result<Option<<T as EntityTrait>::Model>, DbErr> {
        let query_condition =
            QueryCondition::default().with_condition(Condition::all().add(condition.into()));
        let select_query = Self::find_condition_to_select(query_condition);
        let res = select_query.one(&self.db).await?;
        Ok(res)
    }

    pub async fn count_by_condition(
        &self,
        condition: impl Into<QueryCondition>,
    ) -> Result<i64, DbErr> {
        let mut condition: QueryCondition = condition.into();
        condition = condition.clear_cursor().clear_orders();
        let count_query = Self::find_condition_to_select(condition);
        let mut count_query = count_query.select_only().into_query();
        let builder = self.db.get_database_backend();
        count_query.expr_as(
            sea_query::Expr::col(sea_query::Asterisk).count(),
            Alias::new("count"),
        );
        let stmt = builder.build(&count_query);
        let count_res = self.db.query_one(stmt).await?.unwrap();
        let count = count_res.try_get::<i64>("", "count").unwrap();

        Ok(count)
    }

    pub async fn find_all(&self) -> Result<Vec<<T as EntityTrait>::Model>, DbErr> {
        let res = T::find().all(&self.db).await?;
        Ok(res)
    }

    pub async fn update(
        &self,
        model: impl Into<<T as EntityTrait>::ActiveModel>,
    ) -> Result<<T as EntityTrait>::Model, DbErr> {
        let active_model: <T as EntityTrait>::ActiveModel = model.into();
        let res = active_model.update(&self.db).await?;
        Ok(res)
    }

    pub async fn delete_by_id(
        &self,
        id: impl Into<<<T as sea_orm::EntityTrait>::PrimaryKey as PrimaryKeyTrait>::ValueType>,
    ) -> Result<(), DbErr> {
        T::delete_by_id(id).exec(&self.db).await?;
        Ok(())
    }

    pub async fn delete_many(&self, condition: impl IntoCondition) -> Result<(), DbErr> {
        T::delete_many().filter(condition).exec(&self.db).await?;
        Ok(())
    }
}
