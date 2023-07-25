use entity::authors;
use eyre::Result;
use sea_orm_migration::prelude::*;
use sea_orm_migration::sea_orm::entity::*;

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let names = vec!["Unpublished", "One Book", "Multiple Books"];

        for name in names {
            insert_author(name, db).await.unwrap();
        }

        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        // We won't change the database since we can't trust that the names are the same.
        Ok(())
    }
}

async fn insert_author(name: &str, db: &SchemaManagerConnection<'_>) -> Result<()> {
    authors::ActiveModel {
        name: sea_orm::ActiveValue::Set(name.to_owned()),
        ..Default::default()
    }
    .insert(db)
    .await?;
    Ok(())
}
