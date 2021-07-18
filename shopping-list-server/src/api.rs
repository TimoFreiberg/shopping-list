use rocket::{get, post, put, serde::json::Json, State};
use serde::Deserialize;
use time::OffsetDateTime;

use crate::{
    model::{ItemId, Items},
    repo::Repository,
    OpenItem, Result,
};

#[tracing::instrument(skip(repo), err)]
#[get("/items?<offset>&<limit>&<show_done_items>")]
pub async fn get_items(
    repo: &State<Repository>,
    offset: Option<usize>,
    limit: Option<usize>,
    show_done_items: bool,
) -> Result<Json<Items>> {
    let items = repo.get_items(offset, limit, show_done_items).await?;
    Ok(Json(items))
}

#[tracing::instrument(skip(repo), err)]
#[post("/items?<show_done_items>", format = "json", data = "<body>")]
pub async fn add_item(
    repo: &State<Repository>,
    body: Json<AddItemBody>,
    show_done_items: bool,
) -> Result<Json<Items>> {
    let now = OffsetDateTime::now_utc();
    let item = OpenItem {
        id: ItemId::default(),
        name: body.into_inner().name,
        created_at: now,
    };
    repo.add_open_item(item).await?;
    let items = repo.get_items(None, None, show_done_items).await?;
    Ok(Json(items))
}

#[derive(Deserialize, Debug)]
pub struct AddItemBody {
    name: String,
}

#[tracing::instrument(skip(repo), err)]
#[put("/items/<id>/complete?<show_done_items>")]
pub async fn complete_item(
    repo: &State<Repository>,
    id: i64,
    show_done_items: bool,
) -> Result<Json<Items>> {
    let id = ItemId(id);
    repo.complete_item(id, OffsetDateTime::now_utc()).await?;
    let items = repo.get_items(None, None, show_done_items).await?;
    Ok(Json(items))
}

#[tracing::instrument(skip(repo), err)]
#[put("/items/<id>/undo?<show_done_items>")]
pub async fn undo_item(
    repo: &State<Repository>,
    id: i64,
    show_done_items: bool,
) -> Result<Json<Items>> {
    let id = ItemId(id);
    repo.undo_item(id).await?;
    let items = repo.get_items(None, None, show_done_items).await?;
    Ok(Json(items))
}

#[tracing::instrument(skip(repo), err)]
#[put("/items/<id>?<show_done_items>", format = "json", data = "<body>")]
pub async fn edit_item(
    repo: &State<Repository>,
    id: i64,
    body: Json<OpenItem>,
    show_done_items: bool,
) -> Result<Json<Items>> {
    let id = ItemId(id);
    repo.edit_item(id, body.into_inner()).await?;
    let items = repo.get_items(None, None, show_done_items).await?;
    Ok(Json(items))
}
