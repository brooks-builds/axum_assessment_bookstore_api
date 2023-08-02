use crate::models::{author::Author, book::Book};
use eyre::{bail, Result};
use migration::QueryStatementBuilder;
use sea_orm::{
    sea_query::Query, ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    QueryFilter, Set, TryIntoModel,
};

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

pub async fn get_by_id(db: &DatabaseConnection, id: i32) -> Result<Option<Book>> {
    let book = entity::books::Entity::find_by_id(id)
        .find_with_related(entity::authors::Entity)
        .all(db)
        .await?
        .into_iter()
        .map(Into::into)
        .collect::<Vec<Book>>();

    let Some(book) = book.first() else {
        return Ok(None);
    };

    Ok(Some(book.clone()))
}

pub async fn get_books_without_authors(db: &DatabaseConnection) -> Result<Vec<Book>> {
    let books = entity::books::Entity::find()
        .filter(
            Condition::any().add(
                entity::books::Column::Id.not_in_subquery(
                    Query::select()
                        .column(entity::book_authors::Column::BookId)
                        .from(entity::book_authors::Entity)
                        .to_owned(),
                ),
            ),
        )
        .all(db)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok(books)
}

pub async fn delete_many(db: &DatabaseConnection, books: Vec<Book>) -> Result<()> {
    let mut query = entity::books::Entity::delete_many();

    panic!("################################################################################# we are creating an and query, where the book id has to be many at the same time. This obviously won't work so needs to be an in instead");
    for book in books {
        let Some(id) = book.id else { bail!("Error deleting book, missing id"); };

        query = query.filter(entity::books::Column::Id.eq(id));
    }

    query.exec(db).await?;

    Ok(())
}
