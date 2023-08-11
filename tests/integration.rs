mod models;

use crate::models::TestAuthor;
use eyre::Result;

const BASE_URL: &str = "http://localhost:3000";

#[tokio::test]
async fn should_create_an_author() -> Result<()> {
    let new_author = TestAuthor::new_random();
    let url = format!("{BASE_URL}/authors");
    let client = reqwest::Client::new();
    let response = client.post(url).json(&new_author).send().await?;

    assert_eq!(response.status(), 201);

    let created_author = response.json::<TestAuthor>().await?;

    assert!(created_author.id.is_some());
    assert_eq!(created_author.name, new_author.name);
    assert!(created_author.books.is_some_and(|books| books.is_empty()));

    Ok(())
}
