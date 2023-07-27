mod types;

use crate::types::CreateAuthor;
use axum::http::StatusCode;
use axum_assessment_bookstore_api::{
    db::{author_queries::get_author_by_id, connect},
    types::{author::Author, book::Book, ResponseObject},
};
use eyre::Result;
use reqwest::Client;

const BASE_URL: &str = "http://localhost:3000";

#[tokio::test]
async fn create_an_author() -> Result<()> {
    let new_author = CreateAuthor::new_random();
    let url = format!("{BASE_URL}/authors");
    let client = Client::new();
    let response = client.post(url).json(&new_author).send().await?;
    let status = response.status();
    let expected_status = 201;
    let db = connect().await?;

    assert_eq!(status, expected_status);

    let created_author = response.json::<Author>().await?;

    let author_in_db = get_author_by_id(created_author.id, &db).await?;

    assert_eq!(author_in_db, Some(created_author));

    Ok(())
}

#[tokio::test]
async fn get_one_author_with_their_books() -> Result<()> {
    let author_id = 2;
    let url = format!("{BASE_URL}/authors/{author_id}");
    let response = reqwest::get(url).await?;
    let status = response.status();

    assert_eq!(status, StatusCode::OK);

    let author = response
        .json::<ResponseObject<Author>>()
        .await?
        .data
        .unwrap();
    let expected_author = Author {
        id: 2,
        name: "One Book".to_owned(),
        books: vec![Book {
            name: "Free Book".to_owned(),
            price: 0,
            in_stock: true,
        }],
    };

    assert_eq!(author, expected_author);

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
                name: "Free Book".to_owned(),
                price: 0,
                in_stock: true,
            }],
        },
        Author {
            id: 3,
            name: "Multiple Books".to_owned(),
            books: vec![
                Book {
                    name: "Expensive Book".to_owned(),
                    price: 10000,
                    in_stock: true,
                },
                Book {
                    name: "Unavailable Book".to_owned(),
                    price: 1400,
                    in_stock: false,
                },
            ],
        },
    ];

    assert_eq!(authors, expected_authors);

    Ok(())
}

#[tokio::test]
#[ignore = "todo"]
async fn update_an_author() -> Result<()> {
    Ok(())
}

#[tokio::test]
#[ignore = "todo"]
async fn associate_author_with_book() -> Result<()> {
    Ok(())
}

#[tokio::test]
#[ignore = "todo"]
async fn delete_an_author() -> Result<()> {
    Ok(())
}
