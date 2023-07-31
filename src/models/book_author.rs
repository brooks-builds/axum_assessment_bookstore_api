use entity::book_authors::Model;

#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct BookAuthor {
    pub author_id: i32,
    pub book_id: i32,
}

impl From<Model> for BookAuthor {
    fn from(value: Model) -> Self {
        Self {
            author_id: value.author_id,
            book_id: value.book_id,
        }
    }
}
