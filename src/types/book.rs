use entity::books::Model;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Book {
    pub name: String,
    pub price: i32,
    pub in_stock: bool,
}

impl From<&Model> for Book {
    fn from(value: &Model) -> Self {
        Self {
            name: value.name.clone(),
            price: value.price,
            in_stock: value.in_stock,
        }
    }
}
