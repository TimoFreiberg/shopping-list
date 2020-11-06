use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

pub type Result<T> = std::result::Result<T, crate::Error>;

#[derive(Debug)]
#[non_exhaustive]
pub enum Error {}

#[derive(Debug, Serialize, Deserialize)]
pub struct OpenItems {
    pub items: Vec<Item>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletedItems {
    items: Vec<CompletedItem>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    pub(crate) id: ItemId,
    pub(crate) description: String,
    pub created_at: OffsetDateTime,
}

impl Item {
    pub fn complete(self, now: OffsetDateTime) -> CompletedItem {
        CompletedItem {
            id: self.id,
            description: self.description,
            created_at: self.created_at,
            completed_at: now,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CompletedItem {
    id: ItemId,
    description: String,
    created_at: OffsetDateTime,
    completed_at: OffsetDateTime,
}

#[derive(Debug, Clone, Eq, PartialEq, Hash, Serialize, Deserialize)]
pub struct ItemId(pub String);
