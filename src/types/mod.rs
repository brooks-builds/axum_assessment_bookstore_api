pub mod author;
pub mod book;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ResponseObject<JSON: Serialize + Clone> {
    pub status: u16,
    pub data: Option<JSON>,
    pub error: Option<String>,
}

impl<JSON: Serialize + Clone> ResponseObject<JSON> {
    pub fn new_ok(data: Option<JSON>) -> Self {
        Self {
            status: if data.is_some() { 200 } else { 404 },
            data,
            error: None,
        }
    }

    pub fn new_ok_no_content() -> Self {
        Self {
            status: 204,
            data: None,
            error: None,
        }
    }

    pub fn new_created(data: JSON) -> Self {
        Self {
            status: 201,
            data: Some(data),
            error: None,
        }
    }

    pub fn new_internal_error(message: &str) -> Self {
        Self {
            status: 500,
            data: None,
            error: Some(message.to_owned()),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct EmptyResponse;
