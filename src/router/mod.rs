mod author;
mod book;
mod book_author;

use crate::config::AppConfig;
use author::{create_author, delete_author, get_all_authors, get_one_author, update_author};
use axum::{
    routing::{delete, get, post, put},
    Router,
};
use book::{create_book, delete_book, get_all_books, get_one_book, update_book};
use book_author::create_book_author;

pub fn create_router(state: AppConfig) -> Router {
    Router::new()
        .route("/authors", post(create_author))
        .route("/authors", get(get_all_authors))
        .route("/authors/:id", get(get_one_author))
        .route("/authors/:id", put(update_author))
        .route("/authors/:id", delete(delete_author))
        .route("/book_authors", post(create_book_author))
        .route("/books", post(create_book))
        .route("/books", get(get_all_books))
        .route("/books/:id", get(get_one_book))
        .route("/books/:id", put(update_book))
        .route("/books/:id", delete(delete_book))
        .with_state(state)
        .layer(tower_http::trace::TraceLayer::new_for_http())
}
