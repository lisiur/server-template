use sea_orm_migration::{
    prelude::*,
    schema::{boolean, timestamp_with_time_zone},
    sea_orm::ForeignKeyAction,
};

pub struct TableManager<'a> {
    manager: &'a SchemaManager<'a>,
    table_name: String,
    table_ref: TableRef,
    primary_key: Vec<IndexColumn>,
}

impl<'a> TableManager<'a> {
    pub fn new(manager: &'a SchemaManager<'a>, table: impl Iden + 'static) -> Self {
        let table_name = table.to_string();
        Self {
            manager,
            table_ref: table.into_table_ref(),
            table_name,
            primary_key: vec![Shared::Id.into_index_column()],
        }
    }

    pub fn primary_key(mut self, primary_key: Vec<impl Iden + 'static>) -> Self {
        self.primary_key = primary_key
            .into_iter()
            .map(|x| x.into_index_column())
            .collect();
        self
    }

    pub async fn create_table(&self, mut stmt: TableCreateStatement) -> Result<&Self, DbErr> {
        let mut stmt = stmt.table(self.table_ref.clone()).if_not_exists();
        if !self.primary_key.is_empty() {
            let mut primary_index = Index::create();
            let mut primary_index = primary_index.name(format!("pk_{}", self.table_name));
            for col in &self.primary_key {
                primary_index = primary_index.col(col.clone())
            }
            stmt = stmt.primary_key(primary_index);
        }
        self.manager.create_table(stmt.to_owned()).await?;
        self.manager
            .alter_table(
                Table::alter()
                    .table(self.table_ref.clone())
                    .add_column_if_not_exists(boolean(Shared::IsDeleted).default(false))
                    .add_column_if_not_exists(
                        timestamp_with_time_zone(Shared::CreatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .add_column_if_not_exists(
                        timestamp_with_time_zone(Shared::UpdatedAt)
                            .default(Expr::current_timestamp()),
                    )
                    .to_owned(),
            )
            .await?;
        self.create_auto_updated_at_trigger().await?;
        Ok(self)
    }

    pub async fn drop_table(&self) -> Result<&Self, DbErr> {
        self.drop_auto_updated_at_trigger().await?;
        self.manager
            .drop_table(Table::drop().table(self.table_ref.clone()).to_owned())
            .await?;
        Ok(self)
    }

    pub async fn create_foreign_key(
        &self,
        column: impl Iden + 'static,
        foreign_table: impl Iden + 'static,
        foreign_column: impl Iden + 'static,
    ) -> Result<&Self, DbErr> {
        self.manager
            .create_foreign_key(
                ForeignKey::create()
                    .name(format!("fk_{}_{}", self.table_name, column.to_string()))
                    .from(self.table_ref.clone(), column)
                    .to(foreign_table, foreign_column)
                    .on_delete(ForeignKeyAction::NoAction)
                    .on_update(ForeignKeyAction::NoAction)
                    .to_owned(),
            )
            .await?;
        Ok(self)
    }

    async fn create_auto_updated_at_trigger(&self) -> Result<&Self, DbErr> {
        let manager = self.manager;
        let table_name = &self.table_name;
        let function_name = self.update_updated_at_function();
        let trigger_name = self.auto_update_updated_at_trigger();
        let sql = format!(
            r#"
        CREATE OR REPLACE FUNCTION {function_name}
        RETURNS TRIGGER AS $$
        BEGIN
            NEW.updated_at = CURRENT_TIMESTAMP;
            RETURN NEW;
        END;
        $$ LANGUAGE plpgsql;

        CREATE TRIGGER {trigger_name}
        BEFORE UPDATE ON "{table_name}"
        FOR EACH ROW
        EXECUTE FUNCTION {function_name};
        "#,
        );
        manager.get_connection().execute_unprepared(&sql).await?;

        Ok(self)
    }

    async fn drop_auto_updated_at_trigger(&self) -> Result<&Self, DbErr> {
        let table_name = &self.table_name;
        let function_name = self.update_updated_at_function();
        let trigger_name = self.auto_update_updated_at_trigger();
        let sql = format!(
            r#"
        DROP TRIGGER IF EXISTS {trigger_name} ON "{table_name}";
        DROP FUNCTION IF EXISTS {function_name};
        "#,
        );

        self.manager
            .get_connection()
            .execute_unprepared(&sql)
            .await?;

        Ok(self)
    }

    pub async fn create_index(&self, columns: Vec<impl Iden + 'static>) -> Result<&Self, DbErr> {
        let table_name = &self.table_name;
        let index_name = self.columns_index(&columns);
        let column_names = columns
            .iter()
            .map(|c| c.to_string())
            .collect::<Vec<_>>()
            .join(",");
        let sql = format!(
            r#"
        CREATE INDEX IF NOT EXISTS {index_name} ON "{table_name}" ({column_names});
        "#,
        );

        self.manager
            .get_connection()
            .execute_unprepared(&sql)
            .await?;

        Ok(self)
    }

    pub async fn drop_index(&self, columns: Vec<impl Iden + 'static>) -> Result<&Self, DbErr> {
        let index_name = self.columns_index(&columns);
        let sql = format!(
            r#"
        DROP INDEX IF EXISTS {index_name};
        "#,
        );

        self.manager
            .get_connection()
            .execute_unprepared(&sql)
            .await?;

        Ok(self)
    }

    fn update_updated_at_function(&self) -> String {
        format!("update_{}_updated_at()", self.table_name)
    }

    fn auto_update_updated_at_trigger(&self) -> String {
        format!("auto_update_{}_updated_at", self.table_name)
    }

    fn columns_index(&self, columns: &Vec<impl Iden + 'static>) -> String {
        format!(
            "idx_{}_{}",
            self.table_name,
            columns
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join("_")
        )
    }
}

#[derive(DeriveIden)]
pub enum Shared {
    Id,
    IsDeleted,
    CreatedAt,
    UpdatedAt,
}
