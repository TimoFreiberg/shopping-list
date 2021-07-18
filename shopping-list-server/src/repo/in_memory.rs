use std::{
    collections::HashMap,
    sync::{atomic::AtomicI64, Arc, Mutex},
};

use async_trait::async_trait;
use time::OffsetDateTime;

use crate::{
    model::Items,
    repo::{IRepository, DEFAULT_LIMIT},
    DoneItem, ItemId, OpenItem, Result,
};

pub struct InMemoryRepo {
    open_items: Arc<Mutex<HashMap<ItemId, OpenItem>>>,
    done_items: Arc<Mutex<HashMap<ItemId, DoneItem>>>,
    id: AtomicI64,
}

impl Default for InMemoryRepo {
    fn default() -> Self {
        let open_items = Arc::new(Mutex::new(HashMap::<ItemId, OpenItem>::new()));
        let done_items = Arc::new(Mutex::new(HashMap::<ItemId, DoneItem>::new()));
        let id = AtomicI64::new(0);
        Self {
            open_items,
            done_items,
            id,
        }
    }
}

#[async_trait]
impl IRepository for InMemoryRepo {
    async fn get_items(
        &self,
        offset: Option<usize>,
        limit: Option<usize>,
        show_done_items: bool,
    ) -> Result<Items> {
        let open = self.open_items(offset, limit);

        let done = if show_done_items {
            Some(self.done_items(offset, limit))
        } else {
            None
        };

        Ok(Items { open, done })
    }

    async fn add_open_item(&self, mut item: OpenItem) -> Result<()> {
        if item.id.is_zero() {
            item.id = ItemId(self.id.fetch_add(1, std::sync::atomic::Ordering::Relaxed));
        }
        self.open_items.lock().unwrap().insert(item.id, item);
        Ok(())
    }

    async fn complete_item(&self, id: ItemId, now: OffsetDateTime) -> Result<()> {
        let item = self.open_items.lock().unwrap().remove(&id);
        if let Some(item) = item {
            let item = item.complete(now);
            self.done_items.lock().unwrap().insert(item.id, item);
        }
        Ok(())
    }

    async fn undo_item(&self, id: ItemId) -> Result<()> {
        let item = self.done_items.lock().unwrap().remove(&id);
        if let Some(item) = item {
            let item = item.undo();
            self.open_items.lock().unwrap().insert(item.id, item);
        }
        Ok(())
    }

    async fn edit_item(&self, id: ItemId, item: OpenItem) -> Result<()> {
        let mut mutex_guard = self.open_items.lock().unwrap();
        let existing = mutex_guard.get_mut(&id);
        if let Some(existing) = existing {
            existing.name = item.name;
        }
        Ok(())
    }
}

impl InMemoryRepo {
    fn open_items(&self, offset: Option<usize>, limit: Option<usize>) -> Vec<OpenItem> {
        let mutex_guard = self.open_items.lock().unwrap();
        let mut values: Vec<_> = mutex_guard.values().collect();
        values.sort_unstable_by_key(|it| it.created_at);
        values
            .into_iter()
            .skip(offset.unwrap_or(0))
            .take(limit.unwrap_or(DEFAULT_LIMIT))
            .cloned()
            .collect()
    }
    fn done_items(&self, offset: Option<usize>, limit: Option<usize>) -> Vec<DoneItem> {
        let mutex_guard = self.done_items.lock().unwrap();
        let mut values: Vec<_> = mutex_guard.values().collect();
        values.sort_unstable_by_key(|it| it.created_at);
        values
            .into_iter()
            .skip(offset.unwrap_or(0))
            .take(limit.unwrap_or(DEFAULT_LIMIT))
            .cloned()
            .collect()
    }
}
