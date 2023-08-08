use super::author::Author;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct Book {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub price: Option<i32>,
    pub in_stock: Option<bool>,
    pub authors: Option<Vec<Author>>,
}
