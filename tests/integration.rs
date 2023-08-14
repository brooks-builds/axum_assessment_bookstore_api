mod models;

use crate::models::{TestAuthor, TestBookAuthor};
use eyre::Result;
use models::TestBook;
use rand::{
    distributions::{Alphanumeric, DistString},
    thread_rng, Rng,
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

#[tokio::test]
async fn should_delete_an_author() -> Result<()> {
    let new_author = TestAuthor::new_random();
    let authors_url = format!("{BASE_URL}/authors");
    let client = reqwest::Client::new();
    let created_author = client
        .post(authors_url)
        .json(&new_author)
        .send()
        .await?
        .json::<TestAuthor>()
        .await?;
    let one_author_url = format!(
        "{BASE_URL}/authors/{}",
        created_author
            .id
            .expect("created author is supposed to have an id")
    );
    let deleted_response = client.delete(one_author_url.clone()).send().await?;

    assert_eq!(deleted_response.status(), 204);

    let deleted_author_response = reqwest::get(one_author_url).await?;

    assert_eq!(deleted_author_response.status(), 404);

    Ok(())
}

#[tokio::test]
async fn should_create_a_book() -> Result<()> {
    let new_book = TestBook::new_random();
    let url = format!("{BASE_URL}/books");
    let client = reqwest::Client::new();
    let response = client.post(url).json(&new_book).send().await?;

    assert_eq!(response.status(), 201);

    let created_book = response.json::<TestBook>().await?;

    assert!(created_book.id.is_some());
    assert_eq!(created_book.name, new_book.name);
    assert_eq!(created_book.price, new_book.price);
    assert_eq!(created_book.in_stock, new_book.in_stock);
    assert!(created_book
        .authors
        .is_some_and(|authors| authors.is_empty()));

    Ok(())
}

#[tokio::test]
async fn should_get_all_books() -> Result<()> {
    let url = format!("{BASE_URL}/books");
    let books = reqwest::get(url).await?.json::<Vec<TestBook>>().await?;
    let mut seeded_books_found = 0;

    for book in books {
        match book.name.as_str() {
            "Free Book" => {
                seeded_books_found += 1;
                assert!(book.price.is_some_and(|price| price == 0));
                assert!(book.in_stock.is_some_and(|in_stock| in_stock));
                assert!(book.authors.is_some_and(|authors| {
                    authors.len() == 1 && authors[0].name == "One Book"
                }));
            }
            "Expensive Book" => {
                seeded_books_found += 1;
                assert!(book.price.is_some_and(|price| price == 10000));
                assert!(book.in_stock.is_some_and(|in_stock| in_stock));
                assert!(book.authors.is_some_and(|authors| {
                    authors.len() == 1 && authors[0].name == "Multiple Books"
                }));
            }
            "Unavailable Book" => {
                seeded_books_found += 1;
                assert!(book.price.is_some_and(|price| price == 1400));
                assert!(book.in_stock.is_some_and(|in_stock| !in_stock));
                assert!(book.authors.is_some_and(|authors| {
                    authors.len() == 1 && authors[0].name == "Multiple Books"
                }));
            }
            _ => {}
        }
    }

    assert_eq!(seeded_books_found, 3);

    Ok(())
}

#[tokio::test]
async fn should_get_one_book() -> Result<()> {
    let url = format!("{BASE_URL}/books/1");
    let book = reqwest::get(url).await?.json::<TestBook>().await?;

    assert!(book.id.is_some_and(|id| id == 1));
    assert!(book.price.is_some_and(|price| price == 0));
    assert!(book.in_stock.is_some_and(|in_stock| in_stock));
    assert!(book
        .authors
        .is_some_and(|authors| authors.len() == 1 && authors[0].name == "One Book"));

    Ok(())
}

#[tokio::test]
async fn should_update_a_book() -> Result<()> {
    // create book
    let new_book = TestBook::new_random();
    let url = format!("{BASE_URL}/books");
    let client = reqwest::Client::new();
    let response = client.post(url).json(&new_book).send().await?;
    assert_eq!(response.status(), 201);
    let mut created_book = response.json::<TestBook>().await?;

    // update book
    let mut rng = thread_rng();
    let created_book_id = created_book.id.expect("missing book id");
    let url = format!("{BASE_URL}/books/{}", created_book_id);
    created_book.name = Alphanumeric.sample_string(&mut rng, 16);
    created_book.id = Some(rng.gen_range(1..10000));
    created_book.price = Some(rng.gen_range(0..100000));
    created_book.in_stock = Some(rng.gen_bool(0.5));
    let response = client.put(&url).json(&created_book).send().await?;
    assert_eq!(response.status(), 204);

    // verify update happened
    let response = client.get(url).send().await?;
    assert_eq!(response.status(), 200);
    let updated_book = response.json::<TestBook>().await?;
    assert_eq!(
        updated_book.id.expect("missing updated book id"),
        created_book_id
    );
    assert_eq!(updated_book.name, created_book.name);
    assert_eq!(updated_book.price, created_book.price);
    assert_eq!(updated_book.in_stock, created_book.in_stock);

    Ok(())
}

#[tokio::test]
async fn should_delete_a_book() -> Result<()> {
    // create a book
    let new_book = TestBook::new_random();
    let url = format!("{BASE_URL}/books");
    let client = reqwest::Client::new();
    let response = client.post(url).json(&new_book).send().await?;
    assert_eq!(response.status(), 201);
    let created_book = response.json::<TestBook>().await?;
    let book_id = created_book.id.unwrap();

    // delete the book
    let url = format!("{BASE_URL}/books/{book_id}");
    let response = client.delete(&url).send().await?;
    assert_eq!(response.status(), 204);

    // verify book is deleted
    let response = client.get(url).send().await?;
    assert_eq!(response.status(), 404);

    Ok(())
}

#[tokio::test]
async fn should_associate_book_with_author() -> Result<()> {
    // create an author
    let new_author = TestAuthor::new_random();
    let url = format!("{BASE_URL}/authors");
    let client = reqwest::Client::new();
    let response = client.post(url).json(&new_author).send().await?;
    assert_eq!(response.status(), 201);
    let created_author = response.json::<TestAuthor>().await?;
    let created_author_id = created_author.id.unwrap();

    // create a book
    let new_book = TestBook::new_random();
    let url = format!("{BASE_URL}/books");
    let response = client.post(url).json(&new_book).send().await?;
    assert_eq!(response.status(), 201);
    let created_book = response.json::<TestBook>().await?;
    let created_book_id = created_book.id.unwrap();

    // associate book with author
    let book_author = TestBookAuthor {
        author_id: created_author_id,
        book_id: created_book_id,
    };
    let url = format!("{BASE_URL}/book_authors");
    let response = client.post(url).json(&book_author).send().await?;
    assert_eq!(response.status(), 201);

    // when getting author, should also get book they are associated with
    let url = format!("{BASE_URL}/authors/{created_author_id}");
    let response = client.get(url).send().await?;
    assert_eq!(response.status(), 200);
    let author_with_book = response.json::<TestAuthor>().await?;
    let authors_book = author_with_book
        .books
        .as_ref()
        .and_then(|books| books.first())
        .unwrap();
    assert_eq!(authors_book, &created_book);

    // when getting book, should also get author they are associated with
    let url = format!("{BASE_URL}/books/{created_book_id}");
    let response = client.get(url).send().await?;
    assert_eq!(response.status(), 200);
    let book_with_author = response.json::<TestBook>().await?;
    let books_author = book_with_author
        .authors
        .as_ref()
        .and_then(|authors| authors.first())
        .unwrap();
    assert_eq!(books_author, &created_author);

    Ok(())
}

#[tokio::test]
async fn should_remove_association_with_author_when_deleting_book() -> Result<()> {
    // create a author
    let new_author = TestAuthor::new_random();
    let url = format!("{BASE_URL}/authors");
    let client = reqwest::Client::new();
    let response = client.post(url).json(&new_author).send().await?;
    assert_eq!(response.status(), 201);
    let created_author = response.json::<TestAuthor>().await?;
    let created_author_id = created_author.id.unwrap();

    // create a book
    let new_book = TestBook::new_random();
    let url = format!("{BASE_URL}/books");
    let response = client.post(url).json(&new_book).send().await?;
    assert_eq!(response.status(), 201);
    let created_book = response.json::<TestBook>().await?;
    let created_book_id = created_book.id.unwrap();

    // associate author and book
    let url = format!("{BASE_URL}/book_authors");
    let book_authors = TestBookAuthor {
        author_id: created_author_id,
        book_id: created_book_id,
    };
    let response = client.post(url).json(&book_authors).send().await?;
    assert_eq!(response.status(), 201);

    // delete book
    let url = format!("{BASE_URL}/books/{created_book_id}");
    let response = client.delete(url).send().await?;
    assert_eq!(response.status(), 204);

    // get author
    let url = format!("{BASE_URL}/authors/{created_author_id}");
    let response = client.get(url).send().await?;
    assert_eq!(response.status(), 200);
    let author_with_deleted_book = response.json::<TestAuthor>().await?;

    // verify author doesn't have book
    assert!(author_with_deleted_book.books.unwrap().is_empty());

    Ok(())
}

#[tokio::test]
async fn should_remove_association_with_book_when_deleting_author() -> Result<()> {
    // create a author
    let new_author = TestAuthor::new_random();
    let url = format!("{BASE_URL}/authors");
    let client = reqwest::Client::new();
    let response = client.post(url).json(&new_author).send().await?;
    assert_eq!(response.status(), 201);
    let created_author = response.json::<TestAuthor>().await?;
    let created_author_id = created_author.id.unwrap();

    // create a book
    let new_book = TestBook::new_random();
    let url = format!("{BASE_URL}/books");
    let response = client.post(url).json(&new_book).send().await?;
    assert_eq!(response.status(), 201);
    let created_book = response.json::<TestBook>().await?;
    let created_book_id = created_book.id.unwrap();

    // associate author and book
    let url = format!("{BASE_URL}/book_authors");
    let book_authors = TestBookAuthor {
        author_id: created_author_id,
        book_id: created_book_id,
    };
    let response = client.post(url).json(&book_authors).send().await?;
    assert_eq!(response.status(), 201);

    // delete author
    let url = format!("{BASE_URL}/authors/{created_author_id}");
    let response = client.delete(url).send().await?;
    assert_eq!(response.status(), 204);

    // get book
    let url = format!("{BASE_URL}/books/{created_book_id}");
    let response = client.get(url).send().await?;
    assert_eq!(response.status(), 200);
    let book_with_deleted_author = response.json::<TestBook>().await?;

    // verify author doesn't have book
    assert!(book_with_deleted_author.authors.unwrap().is_empty());

    Ok(())
}
