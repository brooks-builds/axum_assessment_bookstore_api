use crate::models::book::Book;
use eyre::{bail, Result};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set, TryIntoModel};

pub async fn insert_book(db: &DatabaseConnection, book: Book) -> Result<Book> {
    let Some(name) = book.name else { bail!("Error creating book, missing name"); };
    let Some(price) = book.price else { bail!("Error creating book, missing price"); };
    let Some(in_stock) = book.in_stock else { bail!("Error creating book, missing in_stock"); };

    Ok(entity::books::ActiveModel {
        name: Set(name),
        price: Set(price),
        in_stock: Set(in_stock),
        ..Default::default()
    }
    .save(db)
    .await?
    .try_into_model()?
    .into())
}
