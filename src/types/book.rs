use entity::books::Model;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Book {
    pub id: i32,
    pub name: String,
    pub price: i32,
    pub in_stock: bool,
}

impl From<&Model> for Book {
    fn from(value: &Model) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
            price: value.price,
            in_stock: value.in_stock,
        }
    }
}
