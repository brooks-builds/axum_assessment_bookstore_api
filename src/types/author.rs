use entity::authors::Model;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateAuthorJson {
    pub name: String,
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Author {
    pub id: i32,
    pub name: String,
}

impl From<Model> for Author {
    fn from(value: Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
        }
    }
}
