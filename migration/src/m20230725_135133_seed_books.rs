use eyre::Result;
use sea_orm_migration::{prelude::*, sea_orm::ActiveModelTrait};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let db = manager.get_connection();
        let books = vec![
            Book {
                name: "Free Book".to_owned(),
                price: 0,
                in_stock: true,
            },
            Book {
                name: "Expensive Book".to_owned(),
                price: 100 * 100,
                in_stock: true,
            },
            Book {
                name: "Unavailable Book".to_owned(),
                price: 1400,
                in_stock: false,
            },
        ];

        for book in books {
            insert_book(db, book).await.unwrap();
        }
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

async fn insert_book(db: &SchemaManagerConnection<'_>, book: Book) -> Result<()> {
    entity::books::ActiveModel {
        name: sea_orm::ActiveValue::Set(book.name),
        price: sea_orm::ActiveValue::Set(book.price),
        in_stock: sea_orm::ActiveValue::Set(book.in_stock),
        ..Default::default()
    }
    .insert(db)
    .await
    .unwrap();
    Ok(())
}

struct Book {
    pub name: String,
    pub price: i32,
    pub in_stock: bool,
}
