use super::{author::Author, ModelsError};
use entity::authors::Model as AuthorModel;
use entity::books::Model as BookModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct Book {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub price: Option<i32>,
    pub in_stock: Option<bool>,
    pub authors: Option<Vec<Author>>,
}

impl From<&BookModel> for Book {
    fn from(value: &BookModel) -> Self {
        Self {
            id: Some(value.id),
            name: Some(value.name.clone()),
            price: Some(value.price),
            in_stock: Some(value.in_stock),
            authors: None,
        }
    }
}

impl From<BookModel> for Book {
    fn from(value: BookModel) -> Self {
        Self {
            id: Some(value.id),
            name: Some(value.name),
            price: Some(value.price),
            in_stock: Some(value.in_stock),
            authors: None,
        }
    }
}

impl From<&(BookModel, Vec<AuthorModel>)> for Book {
    fn from((book, authors): &(BookModel, Vec<AuthorModel>)) -> Self {
        let authors = authors.iter().map(Into::into).collect();

        Self {
            id: Some(book.id),
            name: Some(book.name.clone()),
            price: Some(book.price),
            in_stock: Some(book.in_stock),
            authors: Some(authors),
        }
    }
}

pub struct InsertBook {
    pub name: String,
    pub price: i32,
    pub in_stock: bool,
}

impl TryFrom<Book> for InsertBook {
    type Error = ModelsError;

    fn try_from(value: Book) -> Result<Self, Self::Error> {
        let Some(name) = value.name else {
            return Err(ModelsError::MissingField("name"));
        };
        let Some(price) = value.price else {
            return Err(ModelsError::MissingField("price"));
        };
        let Some(in_stock) = value.in_stock else {
            return Err(ModelsError::MissingField("in_stock"));
        };

        Ok(Self {
            name,
            price,
            in_stock,
        })
    }
}

pub struct UpdateBook {
    pub name: String,
    pub price: i32,
    pub in_stock: bool,
    pub authors: Vec<Author>,
}

impl TryFrom<Book> for UpdateBook {
    type Error = ModelsError;

    fn try_from(value: Book) -> Result<Self, Self::Error> {
        let name = value
            .name
            .ok_or_else(|| ModelsError::MissingField("name"))?;
        let price = value
            .price
            .ok_or_else(|| ModelsError::MissingField("price"))?;
        let in_stock = value
            .in_stock
            .ok_or_else(|| ModelsError::MissingField("in_stock"))?;
        let authors = value
            .authors
            .ok_or_else(|| ModelsError::MissingField("authors"))?;

        Ok(Self {
            name,
            price,
            in_stock,
            authors,
        })
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct BookResponse {
    pub id: i32,
    pub name: String,
    pub price: i32,
    pub in_stock: bool,
    pub authors: Vec<Author>,
}

impl TryFrom<Book> for BookResponse {
    type Error = ModelsError;

    fn try_from(value: Book) -> Result<Self, Self::Error> {
        let id = value.id.ok_or_else(|| ModelsError::MissingField("id"))?;
        let name = value
            .name
            .ok_or_else(|| ModelsError::MissingField("name"))?;
        let price = value
            .price
            .ok_or_else(|| ModelsError::MissingField("price"))?;
        let in_stock = value
            .in_stock
            .ok_or_else(|| ModelsError::MissingField("in_stock"))?;
        let authors = value
            .authors
            .ok_or_else(|| ModelsError::MissingField("authors"))?;

        Ok(Self {
            id,
            name,
            price,
            in_stock,
            authors,
        })
    }
}
