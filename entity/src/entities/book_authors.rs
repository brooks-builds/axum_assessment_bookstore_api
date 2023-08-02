//! `SeaORM` Entity. Generated by sea-orm-codegen 0.11.3

use sea_orm::entity::prelude::*;

#[derive(Clone, Debug, PartialEq, DeriveEntityModel, Eq)]
#[sea_orm(table_name = "book_authors")]
pub struct Model {
    #[sea_orm(primary_key, auto_increment = false)]
    pub author_id: i32,
    #[sea_orm(primary_key, auto_increment = false)]
    pub book_id: i32,
}

#[derive(Copy, Clone, Debug, EnumIter)]
pub enum Relation {
    Books,
    Authors,
}

impl RelationTrait for Relation {
    fn def(&self) -> RelationDef {
        match self {
            Relation::Books => Entity::belongs_to(super::books::Entity)
                .from(Column::BookId)
                .to(super::books::Column::Id)
                .into(),
            Relation::Authors => Entity::belongs_to(super::authors::Entity)
                .from(Column::AuthorId)
                .to(super::authors::Column::Id)
                .into()
        }
    }
}

impl ActiveModelBehavior for ActiveModel {}
