pub mod author;
pub mod book;
pub mod book_author;

#[derive(thiserror::Error, Debug)]
pub enum ModelsError {
    #[error("Missing field: {0}")]
    MissingField(&'static str),
}
