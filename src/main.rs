#[macro_use]
extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::State;
use std::collections::HashMap;
use std::sync::Mutex;

#[derive(Serialize, Deserialize, Clone)]
struct Item {
    id: usize,
    name: String,
}

type ItemStore = Mutex<HashMap<usize, Item>>;

#[post("/items", format = "json", data = "<item>")]
fn create_item(item: Json<Item>, store: &State<ItemStore>) -> Json<Item> {
    let mut items = store.lock().unwrap();
    items.insert(item.id, item.clone().into_inner());
    item
}

#[get("/items/<id>")]
fn get_item(id: usize, store: &State<ItemStore>) -> Option<Json<Item>> {
    let items = store.lock().unwrap();
    items.get(&id).map(|item| Json(item.clone()))
}

#[get("/items")]
fn get_items(store: &State<ItemStore>) -> Json<Vec<Item>> {
    let items = store.lock().unwrap();
    Json(items.values().cloned().collect())
}

#[put("/items/<id>", format = "json", data = "<item>")]
fn update_item(id: usize, item: Json<Item>, store: &State<ItemStore>) -> Option<Json<Item>> {
    let mut items = store.lock().unwrap();
    if items.contains_key(&id) {
        items.insert(id, item.clone().into_inner());
        Some(item)
    } else {
        None
    }
}

#[delete("/items/<id>")]
fn delete_item(id: usize, store: &State<ItemStore>) -> Option<Json<Item>> {
    let mut items = store.lock().unwrap();
    items.remove(&id).map(|item| Json(item))
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .manage(Mutex::new(HashMap::<usize, Item>::new()))
        .mount(
            "/",
            routes![
                index,
                create_item,
                get_item,
                get_items,
                update_item,
                delete_item
            ],
        )
}
