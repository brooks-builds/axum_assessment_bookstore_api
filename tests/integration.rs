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

#[tokio::test]
async fn should_get_all_authors() -> Result<()> {
    let url = format!("{BASE_URL}/authors");
    let authors = reqwest::get(url).await?.json::<Vec<TestAuthor>>().await?;
    let mut seeded_authors_found = 0;

    for author in authors {
        match author.name.as_str() {
            "Unpublished" => {
                seeded_authors_found += 1;
                assert!(author.books.is_some_and(|books| books.is_empty()));
            }
            "One Book" => {
                seeded_authors_found += 1;
                assert!(author
                    .books
                    .is_some_and(|books| books.len() == 1 && books[0].name == "Free Book"));
            }
            "Multiple Books" => {
                seeded_authors_found += 1;
                assert!(author.books.is_some_and(|books| books.len() == 2
                    && (books[0].name == "Expensive Book" || books[0].name == "Unavailable Book")));
            }
            _ => {}
        }
    }

    assert_eq!(seeded_authors_found, 3);

    Ok(())
}
