use eyre::Result;
use sea_orm::{ActiveModelTrait, DatabaseConnection, TryIntoModel};

use crate::models::book_author::BookAuthor;

pub async fn insert_book_author(
    db: &DatabaseConnection,
    book_author: BookAuthor,
) -> Result<BookAuthor> {
    Ok(entity::book_authors::ActiveModel {
        author_id: sea_orm::ActiveValue::Set(book_author.author_id),
        book_id: sea_orm::ActiveValue::Set(book_author.book_id),
    }
    .insert(db)
    .await?
    .try_into_model()?
    .into())
}
