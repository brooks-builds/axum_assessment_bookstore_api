use crate::models::book_author::BookAuthor;
use entity::book_authors::Entity as BooksAuthorEntity;
use entity::book_authors::{ActiveModel as BookAuthorActiveModel, Column};
use eyre::Result;
use sea_orm::{
    ActiveModelBehavior, ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait,
    ModelTrait, QueryFilter, Set,
};

pub async fn associate_book_with_author(
    db: &DatabaseConnection,
    book_author: BookAuthor,
) -> Result<()> {
    let mut book_authors = BookAuthorActiveModel::new();

    book_authors.author_id = Set(book_author.author_id);
    book_authors.book_id = Set(book_author.book_id);
    book_authors.insert(db).await?;

    Ok(())
}

pub async fn delete_book_author_association(
    db: &DatabaseConnection,
    book_author: BookAuthor,
) -> Result<()> {
    let Some(db_book_author) = BooksAuthorEntity::find()
        .filter(Column::AuthorId.eq(book_author.author_id))
        .filter(Column::BookId.eq(book_author.book_id))
        .one(db)
        .await? else {
        return Ok(());
    };

    db_book_author.delete(db).await?;

    Ok(())
}
