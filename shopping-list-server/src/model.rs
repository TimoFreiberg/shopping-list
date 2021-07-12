use std::io::Cursor;

use rocket::{http::Status, response::Responder, Response};
use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

pub type Result<T> = std::result::Result<T, crate::Error>;

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {}

impl<'r, 'o: 'r> Responder<'r, 'o> for Error {
    fn respond_to(self, _request: &'r rocket::Request<'_>) -> rocket::response::Result<'o> {
        let response = "Internal Server Error";
        Response::build()
            .sized_body(response.len(), Cursor::new(response))
            .status(Status::InternalServerError)
            .ok()
    }
}

#[derive(Debug, Serialize)]
pub struct Items {
    pub open: Vec<Item>,
    pub done: Option<Vec<CompletedItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub(crate) id: ItemId,
    pub(crate) name: String,
    pub created_at: OffsetDateTime,
}

impl Item {
    pub fn complete(self, now: OffsetDateTime) -> CompletedItem {
        CompletedItem {
            id: self.id,
            name: self.name,
            created_at: self.created_at,
            done_at: now,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct CompletedItem {
    pub(crate) id: ItemId,
    pub(crate) name: String,
    pub(crate) created_at: OffsetDateTime,
    pub(crate) done_at: OffsetDateTime,
}

impl CompletedItem {
    pub fn undo(self) -> Item {
        Item {
            id: self.id,
            name: self.name,
            created_at: self.created_at,
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ItemId(pub u64);
