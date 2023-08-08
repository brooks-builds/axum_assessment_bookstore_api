use super::book::Book;
use super::ModelsError;
use entity::authors::Model as AuthorModel;
use entity::books::Model as BookModel;
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
            books: None,
        }
    }
}

impl From<&AuthorModel> for Author {
    fn from(value: &AuthorModel) -> Self {
        Self {
            id: Some(value.id),
            name: Some(value.name.clone()),
            books: None,
        }
    }
}

impl From<&(AuthorModel, Vec<BookModel>)> for Author {
    fn from((db_author, db_books): &(AuthorModel, Vec<BookModel>)) -> Self {
        let books = db_books.iter().map(Into::into).collect();

        Self {
            id: Some(db_author.id),
            name: Some(db_author.name.clone()),
            books: Some(books),
        }
    }
}

#[derive(Debug, Clone)]
pub struct CreateAuthor {
    pub name: String,
}

impl TryFrom<Author> for CreateAuthor {
    type Error = ModelsError;

    fn try_from(value: Author) -> Result<Self, Self::Error> {
        let Some(name) = value.name else { return Err(ModelsError::MissingField("name"))};

        Ok(Self { name })
    }
}

#[derive(Debug, Clone)]
pub struct AtomicUpdateAuthor {
    pub name: String,
}

impl TryFrom<Author> for AtomicUpdateAuthor {
    type Error = ModelsError;

    fn try_from(value: Author) -> Result<Self, Self::Error> {
        let Some(name) = value.name else { return Err(ModelsError::MissingField("name"))};

        Ok(Self { name })
    }
}
