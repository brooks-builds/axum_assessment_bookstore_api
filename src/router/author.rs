/// Create an author in the database and return the created author
pub async fn create_author() {
    todo!()
}

/// Take in an id given in the path, and get the author with it's books from the database
pub async fn get_one_author() {
    todo!()
}

/// Get all authors from the database and return them with their associated books
pub async fn get_all_authors() {
    todo!()
}

/// Atomic update an author given the id in the path and an author. Even if the id is passed in with the author JSON don't update the author id in the database
pub async fn update_author() {
    todo!()
}

/// Hard delete the author in the database. Also delete the relationship with the author's books
pub async fn delete_author() {
    todo!()
}
