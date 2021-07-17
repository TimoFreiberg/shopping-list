use std::{
    collections::HashMap,
    sync::{atomic::AtomicU64, Arc, Mutex},
};

use rocket::{fs::FileServer, launch, routes};
use shopping_list_server::{CompletedItem, Item, ItemId};

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Arc::new(Mutex::new(HashMap::<ItemId, Item>::new())))
        .manage(Arc::new(
            Mutex::new(HashMap::<ItemId, CompletedItem>::new()),
        ))
        .manage(AtomicU64::new(0))
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
