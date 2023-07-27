use entity::{authors, books};
use eyre::Result;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, TryIntoModel};

use crate::types::{
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
