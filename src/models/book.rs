use entity::books::Model;
use serde::{Deserialize, Serialize};

use super::author::Author;

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct Book {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub price: Option<i32>,
    pub in_stock: Option<bool>,
    pub authors: Option<Vec<Author>>,
}

impl From<&Model> for Book {
    fn from(value: &Model) -> Self {
        Self {
            id: Some(value.id),
            name: Some(value.name.clone()),
            price: Some(value.price),
            in_stock: Some(value.in_stock),
            authors: Some(vec![]),
        }
    }
}

impl From<Model> for Book {
    fn from(value: Model) -> Self {
        Self {
            id: Some(value.id),
            name: Some(value.name.clone()),
            price: Some(value.price),
            in_stock: Some(value.in_stock),
            authors: Some(vec![]),
        }
    }
}

impl From<(Model, Vec<entity::authors::Model>)> for Book {
    fn from((book, authors): (Model, Vec<entity::authors::Model>)) -> Self {
        Self {
            id: Some(book.id),
            name: Some(book.name),
            price: Some(book.price),
            in_stock: Some(book.in_stock),
            authors: Some(authors.iter().map(Into::into).collect()),
        }
    }
}
