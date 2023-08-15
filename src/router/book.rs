/// Insert a single book into the database and return it to the caller
pub async fn create_book() {
    todo!()
}

/// get a single book from the database and include it's related authors
pub async fn get_one_book() {
    todo!()
}

/// Get all books with their authors from the database
pub async fn get_all_books() {
    todo!()
}

/// Atomic update a book given it's id and book JSON. Don't allow the caller to update the book id in the database even if they pass in the id
pub async fn update_book() {
    todo!()
}

/// Hard delete a book from the database.
///
/// Also delete the relationship between this book and it's authors
pub async fn delete_book() {
    todo!()
}
