use crate::{
    cors::{PreflightCORS, CORS},
    model::{ItemId, Items},
    CompletedItem, Item, Result,
};
use rocket::{get, http::Method, options, post, put, serde::json::Json, State};
use serde::Deserialize;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
};
use time::OffsetDateTime;

#[get("/items?<offset>&<limit>&<done_items_collapsed>")]
pub async fn get_items(
    open_db: &State<Arc<Mutex<HashMap<ItemId, Item>>>>,
    done_db: &State<Arc<Mutex<HashMap<ItemId, CompletedItem>>>>,
    offset: Option<usize>,
    limit: Option<usize>,
    done_items_collapsed: bool,
) -> Result<Json<Items>> {
    let open = open_items(open_db, offset, limit);

    let done = if done_items_collapsed {
        None
    } else {
        Some(done_items(done_db, offset, limit))
    };

    Ok(Json(Items { open, done }))
}

#[post("/items", format = "json", data = "<body>")]
pub async fn add_item(
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

#[put("/items/<id>/complete?<done_items_collapsed>")]
pub async fn complete_item(
    open_db: &State<Arc<Mutex<HashMap<ItemId, Item>>>>,
    done_db: &State<Arc<Mutex<HashMap<ItemId, CompletedItem>>>>,
    id: u64,
    done_items_collapsed: bool,
) -> Result<Json<Items>> {
    let id = ItemId(id);
    let item = open_db.lock().unwrap().remove(&id);
    if let Some(item) = item {
        let now = OffsetDateTime::now_utc();
        let completed_item = item.complete(now);
        done_db.lock().unwrap().insert(id, completed_item);
    }
    get_items(open_db, done_db, None, None, done_items_collapsed).await
}

#[put("/items/<id>/undo?<done_items_collapsed>")]
pub async fn undo_item(
    open_db: &State<Arc<Mutex<HashMap<ItemId, Item>>>>,
    done_db: &State<Arc<Mutex<HashMap<ItemId, CompletedItem>>>>,
    id: u64,
    done_items_collapsed: bool,
) -> Result<Json<Items>> {
    let id = ItemId(id);
    let item = done_db.lock().unwrap().remove(&id);
    if let Some(item) = item {
        let item = item.undo();
        open_db.lock().unwrap().insert(id, item);
    }
    get_items(open_db, done_db, None, None, done_items_collapsed).await
}

#[options("/items/<_..>")]
pub fn cors_preflight<'a>() -> PreflightCORS {
    CORS::any(()).headers(vec!["content-type"]).methods(vec![
        Method::Options,
        Method::Put,
        Method::Post,
        Method::Get,
    ])
}

fn open_items(
    open_db: &State<Arc<Mutex<HashMap<ItemId, Item>>>>,
    offset: Option<usize>,
    limit: Option<usize>,
) -> Vec<Item> {
    let mutex_guard = open_db.lock().unwrap();
    let mut values: Vec<_> = mutex_guard.values().collect();
    values.sort_unstable_by_key(|it| it.created_at);
    values
        .into_iter()
        .skip(offset.unwrap_or(0))
        .take(limit.unwrap_or(DEFAULT_LIMIT))
        .cloned()
        .collect()
}

fn done_items(
    done_db: &State<Arc<Mutex<HashMap<ItemId, CompletedItem>>>>,
    offset: Option<usize>,
    limit: Option<usize>,
) -> Vec<CompletedItem> {
    let mutex_guard = done_db.lock().unwrap();
    let mut values: Vec<_> = mutex_guard.values().collect();
    values.sort_unstable_by_key(|it| it.done_at);
    values
        .into_iter()
        .skip(offset.unwrap_or(0))
        .take(limit.unwrap_or(DEFAULT_LIMIT))
        .cloned()
        .collect()
}

const DEFAULT_LIMIT: usize = 200;
