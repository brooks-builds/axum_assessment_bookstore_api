mod models;

use crate::models::TestAuthor;
use eyre::Result;
use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng,
};

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

#[tokio::test]
async fn should_get_one_author() -> Result<()> {
    let url = format!("{BASE_URL}/authors/2");
    let author = reqwest::get(url).await?.json::<TestAuthor>().await?;

    assert!(author.id.is_some_and(|id| id == 2));
    assert_eq!(author.name, "One Book");
    assert!(author
        .books
        .is_some_and(|books| books[0].name == "Free Book"));

    Ok(())
}

#[tokio::test]
async fn should_update_an_author() -> Result<()> {
    let create_author_url = format!("{BASE_URL}/authors");
    let new_author = TestAuthor::new_random();
    let client = reqwest::Client::new();
    let mut created_author = client
        .post(create_author_url)
        .json(&new_author)
        .send()
        .await?
        .json::<TestAuthor>()
        .await?;
    let mut rng = thread_rng();

    created_author.name = Alphanumeric.sample_string(&mut rng, 16);

    let update_url = format!(
        "{BASE_URL}/authors/{}",
        created_author.id.expect("created author should have an id")
    );
    let update_response = client
        .put(update_url.clone())
        .json(&created_author)
        .send()
        .await?;

    assert_eq!(update_response.status(), 204);

    let after_update_author = client
        .get(update_url)
        .send()
        .await?
        .json::<TestAuthor>()
        .await?;

    assert_eq!(after_update_author.name, created_author.name);

    Ok(())
}
