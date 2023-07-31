use entity::{authors, books};
use eyre::{bail, Result};
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, IntoActiveModel, Set, TryIntoModel,
};

use crate::models::{
    author::{Author, CreateAuthorJson},
    book::Book,
};

pub async fn insert_author(
    create_author: CreateAuthorJson,
    db: &DatabaseConnection,
) -> Result<Author> {
    let author = authors::ActiveModel {
        name: sea_orm::ActiveValue::Set(create_author.name),
        ..Default::default()
    };

    let created_author = author.save(db).await?.try_into_model()?;
    Ok(Author {
        id: created_author.id,
        name: created_author.name,
        books: vec![],
    })
}

pub async fn get_author_by_id(id: i32, db: &DatabaseConnection) -> Result<Option<Author>> {
    let authors = authors::Entity::find_by_id(id)
        .find_with_related(books::Entity)
        .all(db)
        .await?;

    if authors.is_empty() {
        return Ok(None);
    }

    let (author, books) = &authors[0];
    let books = books.iter().map(Into::into).collect::<Vec<Book>>();
    let mut author = Author::from(author);
    author.books = books;

    Ok(Some(author))
}

pub async fn get_all_authors(db: &DatabaseConnection) -> Result<Vec<Author>> {
    let db_authors = authors::Entity::find()
        .find_with_related(books::Entity)
        .all(db)
        .await?;

    Ok(db_authors
        .into_iter()
        .map(|(db_author, db_books)| {
            let mut author = Author::from(&db_author);
            author.books = db_books.iter().map(Into::into).collect();
            author
        })
        .collect())
}

pub async fn update_author(db: &DatabaseConnection, author_id: i32, name: String) -> Result<()> {
    let Some(db_author )= authors::Entity::find_by_id(author_id).one(db).await? else {
        bail!("not_found");
    };
    let mut active_db_author = db_author.into_active_model();

    active_db_author.name = Set(name);
    active_db_author.save(db).await?;

    Ok(())
}
