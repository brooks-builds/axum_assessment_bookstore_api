mod models;

use std::{thread::sleep, time::Duration};

use crate::models::{CreateAuthor, TestBook};
use axum::http::StatusCode;
use axum_assessment_bookstore_api::{
    db::{author_queries::get_author_by_id, connect},
    models::{author::Author, book::Book, ResponseObject},
};
use dotenvy_macro::dotenv;
use eyre::Result;
use sea_orm::{ColumnTrait, Database, DatabaseConnection, EntityTrait, QueryFilter};

const BASE_URL: &str = "http://localhost:3000";

#[tokio::test]
async fn create_an_author() -> Result<()> {
    let mut new_author = CreateAuthor::new_random();

    new_author.create_in_api().await?;

    let db = connect().await?;
    let author_in_db =
        get_author_by_id(new_author.saved.clone().unwrap().data.unwrap().id, &db).await?;

    assert_eq!(author_in_db, new_author.saved.unwrap().data);

    Ok(())
}

#[tokio::test]
async fn get_one_author_with_their_books() -> Result<()> {
    let author = CreateAuthor::new_get_from_api(2).await?;
    let expected_author = Author {
        id: 2,
        name: "One Book".to_owned(),
        books: vec![Book {
            id: Some(1),
            name: Some("Free Book".to_owned()),
            price: Some(0),
            in_stock: Some(true),
            authors: Some(vec![]),
        }],
    };

    assert_eq!(author.saved.unwrap().data.unwrap(), expected_author);

    Ok(())
}

#[tokio::test]
async fn get_all_authors_with_their_books() -> Result<()> {
    let url = format!("{BASE_URL}/authors");
    let response = reqwest::get(url).await?;
    let status = response.status();

    assert_eq!(status, StatusCode::OK);

    let authors = response
        .json::<ResponseObject<Vec<Author>>>()
        .await?
        .data
        .unwrap();

    let mut authors = authors
        .into_iter()
        .filter(|author| author.id <= 3)
        .collect::<Vec<Author>>();
    authors.sort_by(|a, b| a.id.partial_cmp(&b.id).unwrap());
    let expected_authors = vec![
        Author {
            id: 1,
            name: "Unpublished".to_owned(),
            books: vec![],
        },
        Author {
            id: 2,
            name: "One Book".to_owned(),
            books: vec![Book {
                id: Some(1),
                name: Some("Free Book".to_owned()),
                price: Some(0),
                in_stock: Some(true),
                authors: Some(vec![]),
            }],
        },
        Author {
            id: 3,
            name: "Multiple Books".to_owned(),
            books: vec![
                Book {
                    id: Some(2),
                    name: Some("Expensive Book".to_owned()),
                    price: Some(10000),
                    in_stock: Some(true),
                    authors: Some(vec![]),
                },
                Book {
                    id: Some(3),
                    name: Some("Unavailable Book".to_owned()),
                    price: Some(1400),
                    in_stock: Some(false),
                    authors: Some(vec![]),
                },
            ],
        },
    ];

    assert_eq!(authors, expected_authors);

    Ok(())
}

#[tokio::test]
async fn update_an_author() -> Result<()> {
    let mut new_author = CreateAuthor::new_random();

    new_author.create_in_api().await?;
    new_author.change_name();
    new_author.update_in_api().await?;
    new_author.reload_from_api().await?;

    assert_eq!(
        new_author.name,
        new_author.saved.unwrap().data.unwrap().name
    );

    Ok(())
}

#[tokio::test]
async fn associate_author_with_book() -> Result<()> {
    let mut new_author = CreateAuthor::new_random();
    new_author.create_in_api().await?;

    let mut new_book = TestBook::new_random();
    new_book.create_in_api().await?;

    new_author.associate_with_book(&new_book).await?;
    new_author.reload_from_api().await?;

    let author_books = new_author.saved.unwrap().data.unwrap().books;

    assert!(author_books.len() == 1);
    assert_eq!(author_books[0], new_book.api_book.unwrap());

    Ok(())
}

#[tokio::test]
async fn delete_an_author() -> Result<()> {
    let mut author = CreateAuthor::new_random();
    author.create_in_api().await?;

    let mut book = TestBook::new_random();
    book.create_in_api().await?;

    author.associate_with_book(&book).await?;

    author.delete().await?;

    let author_id = author.get_id()?;
    author.reload_from_api().await?;
    assert!(author.saved.unwrap().data.is_none());

    // verify that author no longer associated with book
    let database_uri = dotenv!("DATABASE_URL");
    let db = Database::connect(database_uri).await?;
    let book_author = entity::book_authors::Entity::find()
        .filter(entity::book_authors::Column::AuthorId.eq(author_id))
        .all(&db)
        .await?;

    assert!(book_author.is_empty());

    book.reload_from_api().await?;
    dbg!(&book);
    assert!(book.api_book.is_none());

    Ok(())
}
