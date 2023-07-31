use entity::books::Model;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Book {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub price: Option<i32>,
    pub in_stock: Option<bool>,
}

impl From<&Model> for Book {
    fn from(value: &Model) -> Self {
        Self {
            id: Some(value.id),
            name: Some(value.name.clone()),
            price: Some(value.price),
            in_stock: Some(value.in_stock),
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
        }
    }
}
