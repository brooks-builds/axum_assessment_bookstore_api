mod models;

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
