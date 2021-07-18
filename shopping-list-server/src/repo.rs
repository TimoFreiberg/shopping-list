use async_trait::async_trait;
use time::OffsetDateTime;

use crate::{
    model::{Items, Result},
    ItemId, OpenItem,
};

pub mod in_memory;
pub mod postgres;

pub type Repository = Box<dyn IRepository + Send + Sync + 'static>;

#[async_trait]
pub trait IRepository {
    async fn get_items(
        &self,
        offset: Option<usize>,
        limit: Option<usize>,
        show_done_items: bool,
    ) -> Result<Items>;
    async fn add_open_item(&self, item: OpenItem) -> Result<()>;
    async fn complete_item(&self, id: ItemId, now: OffsetDateTime) -> Result<()>;
    async fn undo_item(&self, id: ItemId) -> Result<()>;
    async fn edit_item(&self, id: ItemId, item: OpenItem) -> Result<()>;
}

const DEFAULT_LIMIT: usize = 200;
