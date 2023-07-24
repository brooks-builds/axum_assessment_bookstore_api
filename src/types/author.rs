use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct CreateAuthorJson {
    pub name: String,
}

pub struct Author {
    pub id: i64,
    pub name: String,
}
