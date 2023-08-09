use crate::models::book::UpdateBook;
use crate::models::book::{Book, InsertBook};
use entity::authors::Entity as AuthorEntity;
use entity::books::ActiveModel as BookActiveModel;
use entity::books::Entity as BookEntity;
use eyre::Result;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, ModelTrait, Set,
    TryIntoModel,
};

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

pub async fn get_all_books(db: &DatabaseConnection) -> Result<Vec<Book>> {
    let books_with_authors = BookEntity::find()
        .find_with_related(AuthorEntity)
        .all(db)
        .await?
        .iter()
        .map(Into::into)
        .collect();

    Ok(books_with_authors)
}

pub async fn update_book(db: &DatabaseConnection, id: i32, book: UpdateBook) -> Result<()> {
    let Some(db_book) = BookEntity::find_by_id(id).one(db).await? else {
        return Ok(())
    };
    let mut db_book = db_book.into_active_model();

    db_book.name = Set(book.name);
    db_book.price = Set(book.price);
    db_book.in_stock = Set(book.in_stock);
    db_book.save(db).await?;

    Ok(())
}

pub async fn delete_book(db: &DatabaseConnection, id: i32) -> Result<()> {
    let Some(book) = BookEntity::find_by_id(id).one(db).await? else {
        return Ok(());
    };

    book.delete(db).await?;

    Ok(())
}
