use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};

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

pub async fn get_one_book(
    state: State<AppConfig>,
    Path(id): Path<i32>,
) -> Result<impl IntoResponse, (StatusCode, Json<ResponseObject<EmptyResponse>>)> {
    let book = book_queries::get_by_id(&state.db, id)
        .await
        .map_err(|error| {
            tracing::error!("Error getting book by id: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseObject::new_internal_error(
                    "Error getting book by id",
                )),
            )
        })?;

    Ok(Json(ResponseObject::new_ok(book)))
}

pub async fn get_all_books(
    state: State<AppConfig>,
) -> Result<impl IntoResponse, (StatusCode, Json<ResponseObject<EmptyResponse>>)> {
    let books = book_queries::get_all(&state.db).await.map_err(|error| {
        tracing::error!("Error getting all books: {error}");

        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(ResponseObject::new_internal_error(
                "Error getting all books",
            )),
        )
    })?;

    Ok(Json(ResponseObject::new_ok(Some(books))))
}

pub async fn update_book(
    state: State<AppConfig>,
    Path(id): Path<i32>,
    Json(book): Json<Book>,
) -> Result<StatusCode, (StatusCode, Json<ResponseObject<EmptyResponse>>)> {
    book_queries::update(&state.db, book, id)
        .await
        .map_err(|error| {
            tracing::error!("Error updating book: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseObject::new_internal_error(
                    "There was an error updating the book",
                )),
            )
        })?;

    Ok(StatusCode::NO_CONTENT)
}
