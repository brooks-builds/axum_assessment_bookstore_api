use crate::models::book_author::BookAuthor;
use entity::book_authors::ActiveModel as BookAuthorActiveModel;
use eyre::Result;
use sea_orm::{ActiveModelBehavior, ActiveModelTrait, DatabaseConnection, Set};

pub async fn associate_book_with_author(
    db: &DatabaseConnection,
    book_author: BookAuthor,
) -> Result<()> {
    let mut book_authors = BookAuthorActiveModel::new();

    book_authors.author_id = Set(book_author.author_id);
    book_authors.book_id = Set(book_author.book_id);
    book_authors.save(db).await?;

    Ok(())
}

pub async fn delete_book_author_association() {}
