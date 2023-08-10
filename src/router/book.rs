use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};

use crate::{
    config::AppConfig,
    db::book_queries,
    models::book::{Book, BookResponse, UpdateBook},
};

use super::ErrorResponse;

pub async fn create_book(
    state: State<AppConfig>,
    Json(book): Json<Book>,
) -> Result<(StatusCode, Json<BookResponse>), ErrorResponse> {
    let book = book.try_into().map_err(|error| {
        tracing::error!("Error converting book into create book: {error}");
        (StatusCode::BAD_REQUEST, format!("{error}"))
    })?;
    let book = book_queries::insert_book(&state.db, book)
        .await
        .map_err(|error| {
            tracing::error!("Error inserting book into db: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was an error creating the book".to_owned(),
            )
        })?
        .try_into()
        .map_err(|error| {
            tracing::error!("Error converting created book into book response: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was an error creating the book".to_owned(),
            )
        })?;

    Ok((StatusCode::CREATED, Json(book)))
}

pub async fn get_one_book(
    state: State<AppConfig>,
    Path(id): Path<i32>,
) -> Result<Json<BookResponse>, ErrorResponse> {
    let book = book_queries::get_book_by_id(&state.db, id)
        .await
        .map_err(|error| {
            tracing::error!("Error getting book from db: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was an error getting the book".to_owned(),
            )
        })?
        .ok_or_else(|| (StatusCode::NOT_FOUND, "Book not found".to_owned()))?
        .try_into()
        .map_err(|error| {
            tracing::error!("Error converting get one book into response book: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was an error getting the book".to_owned(),
            )
        })?;

    Ok(Json(book))
}

pub async fn get_all_books(
    state: State<AppConfig>,
) -> Result<Json<Vec<BookResponse>>, ErrorResponse> {
    let books = book_queries::get_all_books(&state.db)
        .await
        .map_err(|error| {
            tracing::error!("Error getting all books: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was an error getting all books".to_owned(),
            )
        })?
        .into_iter()
        .map(TryInto::try_into)
        .collect::<Result<Vec<BookResponse>, _>>()
        .map_err(|error| {
            tracing::error!("Error converting book to book response: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was an error getting all books".to_owned(),
            )
        })?;

    Ok(Json(books))
}

pub async fn update_book(
    state: State<AppConfig>,
    Path(id): Path<i32>,
    Json(book): Json<Book>,
) -> Result<StatusCode, ErrorResponse> {
    let book = UpdateBook::try_from(book).map_err(|error| {
        tracing::error!("Error converting from request book to update book: {error}");
        (StatusCode::BAD_REQUEST, format!("{error}"))
    })?;

    book_queries::update_book(&state.db, id, book)
        .await
        .map_err(|error| {
            tracing::error!("Error updating book: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was an error updating the book".to_owned(),
            )
        })?;

    Ok(StatusCode::NO_CONTENT)
}

pub async fn delete_book(
    Path(id): Path<i32>,
    State(state): State<AppConfig>,
) -> Result<StatusCode, ErrorResponse> {
    book_queries::delete_book(&state.db, id)
        .await
        .map_err(|error| {
            tracing::error!("Error deleting book: {error}");
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "There was an error deleting the book".to_owned(),
            )
        })?;

    Ok(StatusCode::NO_CONTENT)
}
