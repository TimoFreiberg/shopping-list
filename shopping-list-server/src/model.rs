use std::io::Cursor;

use rocket::{http::Status, response::Responder, Response};
use serde::{Deserialize, Serialize};
use sqlx::migrate::MigrateError;
use thiserror::Error;
use time::OffsetDateTime;

pub type Result<T, E = crate::Error> = std::result::Result<T, E>;

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum Error {
    #[error("DB error: {0:?}")]
    DbError(#[from] sqlx::Error),
    #[error("DB Migration error: {0:?}")]
    MigrateError(#[from] MigrateError),
    #[error("Error: {0:?}")]
    LogicError(#[from] eyre::Error),
}

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
    pub open: Vec<OpenItem>,
    pub done: Option<Vec<DoneItem>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpenItem {
    pub(crate) id: ItemId,
    pub(crate) name: String,
    pub created_at: OffsetDateTime,
}

impl OpenItem {
    pub fn complete(self, now: OffsetDateTime) -> DoneItem {
        DoneItem {
            id: self.id,
            name: self.name,
            created_at: self.created_at,
            done_at: now,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct DoneItem {
    pub(crate) id: ItemId,
    pub(crate) name: String,
    pub(crate) created_at: OffsetDateTime,
    pub(crate) done_at: OffsetDateTime,
}

impl DoneItem {
    pub fn undo(self) -> OpenItem {
        OpenItem {
            id: self.id,
            name: self.name,
            created_at: self.created_at,
        }
    }
}

#[derive(Debug, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ItemId(pub i64);

impl Default for ItemId {
    fn default() -> Self {
        Self(0)
    }
}

impl From<i64> for ItemId {
    fn from(it: i64) -> Self {
        Self(it)
    }
}

impl ItemId {
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}
