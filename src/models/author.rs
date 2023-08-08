use super::book::Book;
use entity::authors::Model as AuthorModel;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone, Default)]
pub struct Author {
    pub id: Option<i32>,
    pub name: Option<String>,
    pub books: Option<Vec<Book>>,
}

impl From<AuthorModel> for Author {
    fn from(value: AuthorModel) -> Self {
        Self {
            id: Some(value.id),
            name: Some(value.name),
            books: Some(vec![]),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreateAuthor {
    pub name: String,
}

impl TryFrom<Author> for CreateAuthor {
    type Error = AuthorError;

    fn try_from(value: Author) -> Result<Self, Self::Error> {
        let Some(name) = value.name else { return Err(AuthorError::MissingAuthorField("name"))};

        Ok(Self { name })
    }
}

#[derive(thiserror::Error, Debug)]
pub enum AuthorError {
    #[error("Missing field: {0}")]
    MissingAuthorField(&'static str),
}
