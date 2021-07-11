use crate::{
    model::{CompletedItems, ItemId, OpenItems},
    CompletedItem, Item, Result,
};
use rocket::{get, post, put, serde::json::Json, State};
use serde::Deserialize;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
};
use time::OffsetDateTime;

#[get("/items?<offset>&<limit>")]
pub async fn get_open_items(
    db: &State<Arc<Mutex<HashMap<ItemId, Item>>>>,
    offset: Option<usize>,
    limit: Option<usize>,
) -> Result<Json<OpenItems>> {
    let mutex_guard = db.lock().unwrap();
    let mut values: Vec<_> = mutex_guard.values().collect();
    values.sort_unstable_by_key(|it| it.created_at);
    let values = values
        .into_iter()
        .skip(offset.unwrap_or(0))
        .take(limit.unwrap_or(DEFAULT_LIMIT))
        .cloned()
        .collect();
    Ok(Json(OpenItems { items: values }))
}

#[get("/items/completed?<offset>&<limit>")]
pub async fn get_done_items(
    db: &State<Arc<Mutex<HashMap<ItemId, CompletedItem>>>>,
    offset: Option<usize>,
    limit: Option<usize>,
) -> Result<Json<CompletedItems>> {
    let mutex_guard = db.lock().unwrap();
    let mut values: Vec<_> = mutex_guard.values().collect();
    values.sort_unstable_by_key(|it| it.completed_at);
    let values = values
        .into_iter()
        .skip(offset.unwrap_or(0))
        .take(limit.unwrap_or(DEFAULT_LIMIT))
        .cloned()
        .collect();
    Ok(Json(CompletedItems { items: values }))
}

#[post("/items", format = "json", data = "<body>")]
pub async fn add_task(
    db: &State<Arc<Mutex<HashMap<ItemId, Item>>>>,
    id_counter: &State<AtomicU64>,
    body: Json<AddTaskBody>,
) -> Result<Json<Item>> {
    let now = OffsetDateTime::now_utc();
    let id = ItemId(id_counter.fetch_add(1, Ordering::Relaxed));
    let item = Item {
        id: id.clone(),
        name: body.into_inner().name,
        created_at: now,
    };
    db.lock().unwrap().insert(id, item.clone());
    Ok(Json(item))
}

#[derive(Deserialize)]
pub struct AddTaskBody {
    name: String,
}

#[put("/items/<id>/complete")]
pub async fn complete_item(
    open_db: &State<Arc<Mutex<HashMap<ItemId, Item>>>>,
    completed_db: &State<Arc<Mutex<HashMap<ItemId, CompletedItem>>>>,
    id: u64,
) -> Result<()> {
    let id = ItemId(id);
    let item = open_db.lock().unwrap().remove(&id);
    if let Some(item) = item {
        let now = OffsetDateTime::now_utc();
        let completed_item = item.complete(now);
        completed_db.lock().unwrap().insert(id, completed_item);
    }
    Ok(())
}

#[put("/items/<id>/undo")]
pub async fn undo_item(
    open_db: &State<Arc<Mutex<HashMap<ItemId, Item>>>>,
    completed_db: &State<Arc<Mutex<HashMap<ItemId, CompletedItem>>>>,
    id: u64,
) -> Result<()> {
    let id = ItemId(id);
    let item = completed_db.lock().unwrap().remove(&id);
    if let Some(item) = item {
        let item = item.undo();
        open_db.lock().unwrap().insert(id, item);
    }
    Ok(())
}

const DEFAULT_LIMIT: usize = 200;
