use crate::models::book::Book;
use eyre::{bail, Result};
use sea_orm::{
    sea_query::Query, ActiveModelTrait, ColumnTrait, Condition, DatabaseConnection, EntityTrait,
    IntoActiveModel, QueryFilter, Set, TryIntoModel,
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
    let query = entity::books::Entity::delete_many();

    let mut condition = Condition::any();

    for book in books {
        let Some(id) = book.id else { bail!("Error deleting book, missing id"); };

        condition = condition.add(entity::books::Column::Id.eq(id));
    }

    query.filter(condition).exec(db).await?;

    Ok(())
}

pub async fn get_all(db: &DatabaseConnection) -> Result<Vec<Book>> {
    Ok(entity::books::Entity::find()
        .find_with_related(entity::authors::Entity)
        .all(db)
        .await?
        .into_iter()
        .map(Into::into)
        .collect())
}

pub async fn update(db: &DatabaseConnection, book: Book, id: i32) -> Result<()> {
    let Some(db_book)= entity::books::Entity::find_by_id(id).one(db).await? else {
        return Ok(());
    };
    let mut db_book = db_book.into_active_model();

    if let Some(name) = book.name {
        db_book.name = Set(name);
    }

    if let Some(price) = book.price {
        db_book.price = Set(price);
    }

    if let Some(in_stock) = book.in_stock {
        db_book.in_stock = Set(in_stock);
    }

    db_book.save(db).await?;
    Ok(())
}

pub async fn delete(db: &DatabaseConnection, id: i32) -> Result<()> {
    entity::books::Entity::delete_by_id(id).exec(db).await?;

    Ok(())
}
