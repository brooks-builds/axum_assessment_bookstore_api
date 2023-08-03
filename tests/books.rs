mod models;

use eyre::Result;
use models::TestBook;

#[tokio::test]
async fn create_book() -> Result<()> {
    let mut book = TestBook::new_random();
    book.create_in_api().await?;

    assert!(book.api_book.is_some());

    Ok(())
}

#[tokio::test]
#[ignore = "todo"]
async fn associate_book_with_author() -> Result<()> {
    Ok(())
}

#[tokio::test]
#[ignore = "todo"]
async fn get_one_book_with_authors() -> Result<()> {
    Ok(())
}

#[tokio::test]
#[ignore = "todo"]
async fn get_all_books_with_their_authors() -> Result<()> {
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
