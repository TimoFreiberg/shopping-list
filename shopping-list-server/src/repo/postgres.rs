use std::path::Path;

use async_trait::async_trait;
use chrono::{DateTime, Utc};
use eyre::eyre;
use sqlx::migrate::Migrator;
use sqlx::{query, PgPool};
use sqlx::{query_as, FromRow};

use crate::{
    model::Items,
    repo::{IRepository, DEFAULT_LIMIT},
    DoneItem, ItemId, OpenItem, Result,
};

pub struct PostgresRepo {
    pool: PgPool,
}

impl PostgresRepo {
    pub async fn new(db_uri: &str) -> Result<Self> {
        let pool = sqlx::PgPool::connect(db_uri).await?;
        let migrator = Migrator::new(Path::new("db/migrations")).await?;
        migrator.run(&pool).await?;
        Ok(Self { pool })
    }
}

#[async_trait]
impl IRepository for PostgresRepo {
    async fn get_items(
        &self,
        offset: Option<usize>,
        limit: Option<usize>,
        show_done_items: bool,
    ) -> Result<crate::model::Items> {
        let open =
            query_as(r#"select * from open_items order by created_at desc limit $1 offset $2"#)
                .bind(limit.unwrap_or(DEFAULT_LIMIT) as u32)
                .bind(offset.unwrap_or(0) as u32)
                .fetch_all(&self.pool)
                .await?
                .into_iter()
                .map(|it: OpenItemRow| OpenItem {
                    id: it.id.into(),
                    name: it.name,
                    created_at: it.created_at,
                })
                .collect();

        let done = if show_done_items {
            let done_items = query_as(r#"select * from done_items order by done_at desc"#)
                .fetch_all(&self.pool)
                .await?
                .into_iter()
                .map(|it: DoneItemRow| DoneItem {
                    id: it.id.into(),
                    name: it.name,
                    created_at: it.created_at,
                    done_at: it.done_at,
                })
                .collect();
            Some(done_items)
        } else {
            None
        };
        Ok(Items { open, done })
    }

    async fn add_open_item(&self, item: OpenItem) -> Result<()> {
        if !item.id.is_zero() {
            query(r#"insert into open_items (id, name, created_at) values ($1, $2, $3)"#)
                .bind(&item.id.0)
                .bind(&item.name)
                .bind(&item.created_at)
                .execute(&self.pool)
                .await?;
        } else {
            query(r#"insert into open_items (name, created_at) values ($1, $2)"#)
                .bind(&item.name)
                .bind(&item.created_at)
                .execute(&self.pool)
                .await?;
        }

        Ok(())
    }

    async fn complete_item(&self, id: ItemId, now: DateTime<Utc>) -> Result<()> {
        let tx = self.pool.begin().await?;
        let item = self.delete_open_item(&id).await?;
        if let Some(item) = item {
            let item = item.complete(now);
            self.add_done_item(&item).await?;
        }
        tx.commit().await?;
        Ok(())
    }

    async fn undo_item(&self, id: ItemId) -> Result<()> {
        let tx = self.pool.begin().await?;
        let item = self.delete_done_item(&id).await?;
        if let Some(item) = item {
            let item = item.undo();
            self.add_open_item(item).await?;
        }
        tx.commit().await?;
        Ok(())
    }

    async fn edit_item(&self, id: ItemId, item: OpenItem) -> Result<()> {
        query(r#"update open_items set name = $1 where id = $2"#)
            .bind(&item.name)
            .bind(&id.0)
            .execute(&self.pool)
            .await?;
        Ok(())
    }
}

impl PostgresRepo {
    async fn add_done_item(&self, item: &DoneItem) -> Result<()> {
        if item.id.is_zero() {
            Err(eyre!("Can't create new done item without ID").into())
        } else {
            query(
                r#"insert into done_items (id, name, created_at, done_at) values ($1, $2, $3, $4)"#,
            )
            .bind(&item.id.0)
            .bind(&item.name)
            .bind(&item.created_at)
            .bind(&item.done_at)
            .execute(&self.pool)
            .await?;
            Ok(())
        }
    }
    async fn delete_open_item(&self, id: &ItemId) -> Result<Option<OpenItem>> {
        let deleted = query_as(r#"delete from open_items where id = $1 returning *"#)
            .bind(id.0)
            .fetch_optional(&self.pool)
            .await?
            .map(|it: OpenItemRow| OpenItem {
                id: it.id.into(),
                name: it.name,
                created_at: it.created_at,
            });
        Ok(deleted)
    }

    async fn delete_done_item(&self, id: &ItemId) -> Result<Option<DoneItem>> {
        let deleted = query_as(r#"delete from done_items where id = $1 returning *"#)
            .bind(id.0)
            .fetch_optional(&self.pool)
            .await?
            .map(|it: DoneItemRow| DoneItem {
                id: it.id.into(),
                name: it.name,
                created_at: it.created_at,
                done_at: it.done_at,
            });
        Ok(deleted)
    }
}

#[derive(FromRow)]
struct OpenItemRow {
    id: i64,
    name: String,
    created_at: DateTime<Utc>,
}

#[derive(FromRow)]
struct DoneItemRow {
    id: i64,
    name: String,
    created_at: DateTime<Utc>,
    done_at: DateTime<Utc>,
}
