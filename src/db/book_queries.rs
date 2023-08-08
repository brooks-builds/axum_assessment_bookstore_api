use crate::models::book::{Book, InsertBook};
use entity::books::ActiveModel as BookActiveModel;
use eyre::Result;
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TryIntoModel};

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

pub async fn get_book_by_id() {}

pub async fn get_all_books() {}

pub async fn update_book() {}

pub async fn delete_book() {}
