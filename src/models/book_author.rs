#[derive(serde::Deserialize, serde::Serialize, Debug, Clone)]
pub struct BookAuthor {
    pub author_id: i32,
    pub book_id: i32,
}
