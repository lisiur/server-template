//! `SeaORM` Entity, @generated by sea-orm-codegen 1.1.10

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "applications")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub id: Uuid,
    pub app_id: Uuid,
    pub app_secret: String,
    pub name: String,
    pub description: Uuid,
    pub is_deleted: bool,
    pub created_at: DateTimeWithTimeZone,
    pub updated_at: DateTimeWithTimeZone,
}

#[derive(Copy, Clone, Debug, EnumIter, DeriveRelation)]
pub enum Relation {
    #[sea_orm(has_many = "super::menus::Entity")]
    Menus,
}

impl Related<super::menus::Entity> for Entity {
    fn to() -> RelationDef {
        Relation::Menus.def()
    }
}

impl ActiveModelBehavior for ActiveModel {}
