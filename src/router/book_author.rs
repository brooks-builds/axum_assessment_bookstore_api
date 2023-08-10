use axum::{extract::State, http::StatusCode, Json};

use crate::{config::AppConfig, db::book_author_queries, models::book_author::BookAuthor};

use super::ErrorResponse;

pub async fn create_book_author(
    state: State<AppConfig>,
    Json(book_author): Json<BookAuthor>,
) -> Result<StatusCode, ErrorResponse> {
    book_author_queries::associate_book_with_author(&state.db, book_author)
        .await
        .map_err(|error| {
            tracing::error!("Error creating association between book and author: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was an error associating the book and author".to_owned(),
            )
        })?;

    Ok(StatusCode::CREATED)
}
