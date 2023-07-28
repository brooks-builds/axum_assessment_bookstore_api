use entity::authors::Model;
use serde::{Deserialize, Serialize};

use super::book::Book;

#[derive(Serialize, Deserialize)]
pub struct CreateAuthorJson {
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
pub struct Author {
    pub id: i32,
    pub name: String,
    pub books: Vec<Book>,
}

impl From<&Model> for Author {
    fn from(value: &Model) -> Self {
        Self {
            id: value.id,
            name: value.name.clone(),
            books: vec![],
        }
    }
}
