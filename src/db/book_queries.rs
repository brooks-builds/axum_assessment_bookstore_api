use crate::models::book::{Book, InsertBook};
use entity::authors::Entity as AuthorEntity;
use entity::books::ActiveModel as BookActiveModel;
use entity::books::Entity as BookEntity;
use eyre::Result;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set, TryIntoModel};

pub async fn insert_book(db: &DatabaseConnection, book: InsertBook) -> Result<Book> {
    let book = BookActiveModel {
        name: Set(book.name),
        price: Set(book.price),
        in_stock: Set(book.in_stock),
        ..Default::default()
    }
    .save(db)
    .await?
    .try_into_model()?;

    Ok(book.into())
}

pub async fn get_book_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<Book>> {
    let books = BookEntity::find_by_id(id)
        .find_with_related(AuthorEntity)
        .all(db)
        .await?;
    let Some(book) = books.first() else {
        return Ok(None);
    };

    Ok(Some(book.into()))
}

pub async fn get_all_books() {}

pub async fn update_book() {}

pub async fn delete_book() {}
