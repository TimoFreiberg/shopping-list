use crate::{
    model::{ItemId, OpenItems},
    Item, Result,
};
use rocket::{get, post, put, State};
use rocket_contrib::json::Json;
use serde::Deserialize;
use std::{
    collections::HashMap,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Mutex,
    },
};
use time::OffsetDateTime;

#[get("/items?<offset>&<limit>")]
pub fn get_open_items(
    db: State<Arc<Mutex<HashMap<ItemId, Item>>>>,
    offset: Option<usize>,
    limit: Option<usize>,
) -> Result<Json<OpenItems>> {
    let mut values: Vec<_> = db.lock().unwrap().values().cloned().collect();
    values.sort_unstable_by_key(|it| it.created_at);
    values = values
        .into_iter()
        .skip(offset.unwrap_or(0))
        .take(limit.unwrap_or(DEFAULT_LIMIT))
        .collect();
    Ok(Json(OpenItems { items: values }))
}

#[post("/items", format = "json", data = "<body>")]
pub fn add_task(
    db: State<Arc<Mutex<HashMap<ItemId, Item>>>>,
    id_counter: State<AtomicUsize>,
    body: Json<AddTaskBody>,
) -> Result<()> {
    let now = OffsetDateTime::now_utc();
    let id = ItemId(id_counter.fetch_add(1, Ordering::Relaxed).to_string());
    let item = Item {
        id: id.clone(),
        description: body.description.clone(),
        created_at: now,
    };
    db.lock().unwrap().insert(id, item);
    Ok(())
}

#[derive(Deserialize)]
pub struct AddTaskBody {
    description: String,
}

#[put("/items/<id>/complete")]
pub fn complete_item(db: State<Arc<Mutex<HashMap<ItemId, Item>>>>, id: String) -> Result<()> {
    let id = ItemId(id);
    let _item = db.lock().unwrap().remove(&id);
    Ok(())
}

const DEFAULT_LIMIT: usize = 100;
