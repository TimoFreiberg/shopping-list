use std::env;

use rocket::{fs::FileServer, launch, routes};
use shopping_list_server::{InMemoryRepo, PostgresRepo, Repository};
use tracing::info;

#[launch]
async fn rocket() -> _ {
    dotenv::dotenv().ok();

    tracing_subscriber::fmt::init();

    let repo: Repository = if env::var("IN_MEMORY_DB").is_ok() {
        info!("Using in-memory database");
        Box::new(InMemoryRepo::default())
    } else {
        info!("Using Postgres");
        let db_uri = env::var("DATABASE_URL").expect("DATABASE_URL environment variable not set");
        let repo = PostgresRepo::new(&db_uri)
            .await
            .expect("Connecting to database failed");
        Box::new(repo)
    };

    rocket::build()
        .manage(repo)
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
