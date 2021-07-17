use std::{collections::HashMap, env, sync::{atomic::AtomicU64, Arc, Mutex}};

use rocket::{fs::FileServer, launch, routes};
use shopping_list_server::{CompletedItem, Item, ItemId};

#[launch]
async fn rocket() -> _ {
    dotenv::dotenv().ok();

    let db_uri = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
    let db_connection = sqlx::PgPool::connect(&db_uri).await.expect("Database connection failed");

    rocket::build()
        .manage(Arc::new(Mutex::new(HashMap::<ItemId, Item>::new())))
        .manage(Arc::new(
            Mutex::new(HashMap::<ItemId, CompletedItem>::new()),
        ))
        .manage(AtomicU64::new(0))
        .manage(db_connection)
        .mount(
            "/",
            routes![
                shopping_list_server::api::get_items,
                shopping_list_server::api::add_item,
                shopping_list_server::api::complete_item,
                shopping_list_server::api::undo_item,
                shopping_list_server::api::edit_item,
            ],
        )
        .mount("/", FileServer::from("site"))
}
