use sqlx::{query, PgPool};
use sqlx::{query_as, FromRow};
use time::OffsetDateTime;

use crate::{Item, ItemId, Result};

#[derive(FromRow)]
struct ItemRow {
    id: u32,
    name: String,
    created_at: OffsetDateTime,
    done_at: Option<OffsetDateTime>,
}

pub(crate) async fn add_item(pool: &PgPool, item: Item) -> Result<()> {
    query(r#"insert into open_items (name, created_at) values ($1, $2) returning id"#)
        .bind(&item.name)
        .bind(&item.created_at)
        .execute(pool)
        .await?;
    Ok(())
}

pub(crate) async fn get_open_items(pool: &PgPool) -> Result<Vec<Item>> {
    let result: Vec<ItemRow> = query_as(r#"select * from open_items order by created_at"#)
        .fetch_all(pool)
        .await?;
    Ok(result
        .into_iter()
        .map(|it| Item {
            id: ItemId(it.id.into()),
            name: it.name,
            created_at: it.created_at,
        })
        .collect())
}
