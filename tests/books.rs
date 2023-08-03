mod models;

use axum_assessment_bookstore_api::models::{book::Book, ResponseObject};
use eyre::Result;
use models::TestBook;

#[tokio::test]
async fn get_one_book_with_authors() -> Result<()> {
    let book = TestBook::new_from_api(1).await?;
    let author = book
        .api_book
        .unwrap()
        .authors
        .unwrap()
        .first()
        .unwrap()
        .clone();

    assert_eq!(author.name, "One Book".to_owned());
    Ok(())
}

#[tokio::test]
async fn get_all_books_with_their_authors() -> Result<()> {
    let url = "http://localhost:3000/books";
    let response = reqwest::get(url).await?;
    let status = response.status();

    assert_eq!(status, 200);

    let books = response
        .json::<ResponseObject<Vec<Book>>>()
        .await?
        .data
        .unwrap();

    assert!(books.len() > 3);

    let book = books
        .iter()
        .find(|book| book.id.is_some_and(|id| id == 1))
        .unwrap();
    let author = book.authors.as_ref().unwrap().first().unwrap();
    assert_eq!(author.name, "One Book".to_owned());

    Ok(())
}

#[tokio::test]
#[ignore = "todo"]
async fn update_book() -> Result<()> {
    Ok(())
}

#[tokio::test]
#[ignore = "todo"]
async fn delete_book() -> Result<()> {
    Ok(())
}
