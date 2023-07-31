use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    config::AppConfig,
    db::book_author_queries,
    models::{book_author::BookAuthor, EmptyResponse, ResponseObject},
};

pub async fn create_book_author(
    state: State<AppConfig>,
    Json(book_author): Json<BookAuthor>,
) -> Result<impl IntoResponse, (StatusCode, Json<ResponseObject<EmptyResponse>>)> {
    let book_author = book_author_queries::insert_book_author(&state.db, book_author)
        .await
        .map_err(|error| {
            tracing::error!("Error inserting book author: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseObject::new_internal_error(
                    "Error creating association between book and author",
                )),
            )
        })?;

    Ok((
        StatusCode::CREATED,
        Json(ResponseObject::new_created(book_author)),
    ))
}
