use eyre::Result;
use sea_orm_migration::{prelude::*, sea_orm::ActiveModelTrait};

#[derive(DeriveMigrationName)]
pub struct Migration;

#[async_trait::async_trait]
impl MigrationTrait for Migration {
    async fn up(&self, manager: &SchemaManager) -> Result<(), DbErr> {
        let book_authors = vec![
            BookAuthor {
                author_id: 2,
                book_id: 1,
            },
            BookAuthor {
                author_id: 3,
                book_id: 2,
            },
            BookAuthor {
                author_id: 3,
                book_id: 3,
            },
        ];
        let db = manager.get_connection();
        for book_author in book_authors {
            insert_book_author(book_author, db).await.unwrap();
        }
        Ok(())
    }

    async fn down(&self, _manager: &SchemaManager) -> Result<(), DbErr> {
        Ok(())
    }
}

struct BookAuthor {
    pub author_id: i32,
    pub book_id: i32,
}

async fn insert_book_author(
    book_author: BookAuthor,
    db: &SchemaManagerConnection<'_>,
) -> Result<()> {
    entity::book_authors::ActiveModel {
        author_id: sea_orm::ActiveValue::Set(book_author.author_id),
        book_id: sea_orm::ActiveValue::Set(book_author.book_id),
    }
    .insert(db)
    .await?;
    Ok(())
}
