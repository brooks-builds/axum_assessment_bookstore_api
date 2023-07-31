use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};

use crate::{
    config::AppConfig,
    db::book_queries,
    models::{book::Book, EmptyResponse, ResponseObject},
};

pub async fn create_book(
    state: State<AppConfig>,
    Json(book): Json<Book>,
) -> Result<impl IntoResponse, (StatusCode, Json<ResponseObject<EmptyResponse>>)> {
    let book = book_queries::insert_book(&state.db, book)
        .await
        .map_err(|error| {
            tracing::error!("Error creating book: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseObject::new_internal_error(
                    "There was an error creating the book",
                )),
            )
        })?;
    Ok((StatusCode::CREATED, Json(ResponseObject::new_created(book))))
}
