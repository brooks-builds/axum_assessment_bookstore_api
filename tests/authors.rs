mod types;

use crate::types::{CreateAuthor, CreateAuthorResponse};
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

    assert_eq!(status, expected_status);

    let created_author = response.json::<CreateAuthorResponse>().await?;

    Ok(())
}

#[tokio::test]
#[ignore = "todo"]
async fn get_one_author_with_their_books() -> Result<()> {
    Ok(())
}

#[tokio::test]
#[ignore = "todo"]
async fn get_all_authors() -> Result<()> {
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
