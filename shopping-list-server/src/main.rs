use std::{
    collections::HashMap,
    sync::{atomic::AtomicUsize, Arc, Mutex},
};

use rocket::routes;
use shopping_list_server::{Item, ItemId};

fn main() {
    rocket::ignite()
        .manage(Arc::new(Mutex::new(HashMap::<ItemId, Item>::new())))
        .manage(AtomicUsize::new(0))
        .mount(
            "/",
            routes![
                shopping_list_server::api::get_open_items,
                shopping_list_server::api::add_task,
                shopping_list_server::api::complete_item
            ],
        )
        .launch();
}
