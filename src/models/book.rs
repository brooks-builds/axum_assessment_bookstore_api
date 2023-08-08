use super::author::Author;
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
