#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct CreateBookAuthor {
    pub author_id: i32,
    pub book_id: i32,
}
