use axum::http::StatusCode;
use axum_assessment_bookstore_api::models::{
    author::Author, book::Book, book_author::BookAuthor, ResponseObject,
};
use eyre::{bail, eyre, Result};
use rand::{
    distributions::{Alphanumeric, DistString},
    Rng,
};
use serde::{Deserialize, Serialize};

const BASE_URL: &str = "http://localhost:3000";

#[derive(Serialize, Deserialize, Debug)]
pub struct CreateAuthor {
    pub name: String,
    #[serde(skip)]
    pub saved: Option<ResponseObject<Author>>,
}

impl CreateAuthor {
    pub fn new_random() -> Self {
        let mut rng = rand::thread_rng();
        let name = Alphanumeric.sample_string(&mut rng, 8);

        Self { name, saved: None }
    }

    pub async fn new_get_from_api(author_id: i32) -> Result<Self> {
        let url = format!("{BASE_URL}/authors/{author_id}");
        let response = reqwest::get(url).await?;
        let status = response.status();

        assert_eq!(status, StatusCode::OK);

        let saved = response.json::<ResponseObject<Author>>().await?;
        let name = saved.data.as_ref().unwrap().name.clone();

        Ok(Self {
            name,
            saved: Some(saved),
        })
    }

    pub fn change_name(&mut self) {
        let mut rng = rand::thread_rng();
        self.name = Alphanumeric.sample_string(&mut rng, 8);
    }

    pub async fn create_in_api(&mut self) -> Result<()> {
        let url = format!("{BASE_URL}/authors");
        let client = reqwest::Client::new();
        let response = client.post(url).json(&self).send().await?;

        assert_eq!(response.status(), StatusCode::CREATED);

        self.saved = response.json().await?;

        Ok(())
    }

    pub async fn update_in_api(&mut self) -> Result<()> {
        let author_id = self.saved.as_ref().unwrap().data.as_ref().unwrap().id;
        let url = format!("{BASE_URL}/authors/{author_id}");
        let client = reqwest::Client::new();
        let response = client.put(url).json(&self).send().await?;
        let status = response.status();

        assert_eq!(status, StatusCode::NO_CONTENT);

        Ok(())
    }

    pub async fn reload_from_api(&mut self) -> Result<()> {
        let author_id = self.saved.as_ref().unwrap().data.as_ref().unwrap().id;
        let url = format!("{BASE_URL}/authors/{author_id}");
        let response = reqwest::get(url).await?;
        self.saved = response.json().await?;

        Ok(())
    }

    pub async fn associate_with_book(&self, book: &TestBook) -> Result<()> {
        let author_id = self.saved.as_ref().unwrap().data.as_ref().unwrap().id;
        let book_id = book.api_book.as_ref().unwrap().id.unwrap();
        let url = format!("{BASE_URL}/book_authors");
        let book_author = BookAuthor { author_id, book_id };
        let client = reqwest::Client::new();
        let response = client.post(url).json(&book_author).send().await?;

        assert_eq!(response.status(), 201);

        Ok(())
    }

    pub async fn delete(&mut self) -> Result<()> {
        let Some(author) = &self.saved else { bail!("Missing author"); };
        let Some(author) = &author.data else { bail!("Missing data in author object"); };
        let id = author.id;

        let url = format!("{BASE_URL}/authors/{id}");
        let client = reqwest::Client::new();
        let response = client.delete(url).send().await?;
        let status = response.status();

        assert_eq!(status, 204);

        Ok(())
    }

    pub fn get_id(&self) -> Result<i32> {
        let Some(author) = &self.saved else { bail!("Missing author"); };
        let Some(author) = &author.data else { bail!("Missing data in author object"); };
        let id = author.id;

        Ok(id)
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TestBook {
    #[serde(skip)]
    pub api_book: Option<Book>,
    pub name: String,
    pub price: i32,
    pub in_stock: bool,
}

impl TestBook {
    pub fn new_random() -> Self {
        let mut rng = rand::thread_rng();
        let name = Alphanumeric.sample_string(&mut rng, 8);
        let price = rng.gen_range(0..10000);
        let in_stock = rng.gen_bool(0.5);

        Self {
            api_book: None,
            name,
            price,
            in_stock,
        }
    }

    pub async fn new_from_api(book_id: i32) -> Result<Self> {
        let url = format!("{BASE_URL}/books/{book_id}");
        let response = reqwest::get(url).await?;
        let status = response.status();

        assert_eq!(status, 200);

        let book = response
            .json::<ResponseObject<Book>>()
            .await?
            .data
            .ok_or_else(|| {
                eyre!("Book doesn't exist when creating a test book from an api book")
            })?;

        Ok(Self {
            api_book: Some(book.clone()),
            name: book.name.unwrap(),
            price: book.price.unwrap(),
            in_stock: book.in_stock.unwrap(),
        })
    }

    pub async fn create_in_api(&mut self) -> Result<()> {
        let url = format!("{BASE_URL}/books");
        let client = reqwest::Client::new();
        let response = client.post(url).json(&self).send().await?;

        assert_eq!(response.status(), 201);

        let response_data = response.json::<ResponseObject<Book>>().await?;

        self.api_book = response_data.data;

        Ok(())
    }

    pub async fn reload_from_api(&mut self) -> Result<()> {
        let Some(api_book) = &self.api_book else { bail!("Missing book"); };
        let Some(id) = api_book.id else { bail!("Missing book id"); };
        let url = format!("{BASE_URL}/books/{id}");
        let response = reqwest::get(url).await?;

        assert_eq!(response.status(), 200);

        let book = response.json::<ResponseObject<Book>>().await?;
        self.api_book = book.data;

        Ok(())
    }

    pub async fn update_in_api(&self) -> Result<()> {
        let id = self.api_book.as_ref().unwrap().id.unwrap();
        let url = format!("{BASE_URL}/books/{id}");
        let client = reqwest::Client::new();
        let response = client.put(url).json(self).send().await?;
        let status = response.status();

        assert_eq!(status, 204);

        Ok(())
    }
}
