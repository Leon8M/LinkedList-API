#[macro_use] extern crate rocket;

use rocket::serde::{json::Json, Deserialize, Serialize};
use rocket::http::Status;
use rocket::response::status;
use rocket::fairing::{Fairing, Info, Kind};
use rocket::http::Header;
use std::sync::Mutex;

// Define a CORS Fairing for frontend communication
pub struct CORS;


#[options("/<path..>")]
fn options(path: rocket::http::uri::Segments<'_, rocket::http::uri::fmt::Path>) -> rocket::response::status::NoContent {
    rocket::response::status::NoContent
}



#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "CORS Fairing",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, req: &'r rocket::Request<'_>, res: &mut rocket::Response<'r>) {
        if req.method() == rocket::http::Method::Options {
            res.set_status(Status::NoContent);
        }

        res.set_header(Header::new("Access-Control-Allow-Origin", "http://localhost:5173")); // Set to frontend's URL
        res.set_header(Header::new("Access-Control-Allow-Methods", "GET, POST, OPTIONS"));
        res.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type"));
    }
}


// Define the Node and LinkedList structures
#[derive(Serialize)]
struct Node {
    data: i32,
    next_node: Option<Box<Node>>,
}

impl Node {
    fn new(data: i32) -> Self {
        Node { data, next_node: None }
    }
}

#[derive(Serialize)]
struct LinkedList {
    head: Option<Box<Node>>,
}

impl LinkedList {
    fn new() -> Self {
        LinkedList { head: None }
    }

    fn size(&self) -> usize {
        let mut count = 0;
        let mut current = &self.head;
        while let Some(node) = current {
            count += 1;
            current = &node.next_node;
        }
        count
    }

    fn add(&mut self, data: i32) {
        let mut new_node = Box::new(Node::new(data));
        new_node.next_node = self.head.take();
        self.head = Some(new_node);
    }

    fn search(&self, key: i32) -> Option<&Node> {
        let mut current = &self.head;
        while let Some(node) = current {
            if node.data == key {
                return Some(node);
            }
            current = &node.next_node;
        }
        None
    }

    fn insert(&mut self, data: i32, index: usize) -> Result<(), String> {
        if index == 0 {
            self.add(data);
            return Ok(());
        }

        let mut new_node = Box::new(Node::new(data));
        let mut current = &mut self.head;
        let mut position = 0;

        while let Some(node) = current {
            if position == index - 1 {
                new_node.next_node = node.next_node.take();
                node.next_node = Some(new_node);
                return Ok(());
            }
            current = &mut node.next_node;
            position += 1;
        }

        Err("Index out of bounds".to_string())
    }

    fn remove(&mut self, key: i32) -> bool {
        let mut current = &mut self.head;
        loop {
            match current {
                Some(node) if node.data == key => {
                    *current = node.next_node.take();
                    return true;
                }
                Some(node) => {
                    current = &mut node.next_node;
                }
                None => break,
            }
        }
        false
    }
}

// Define a wrapper around LinkedList to make it thread-safe
struct AppState {
    list: Mutex<LinkedList>,
}

// API Endpoints



#[get("/")]
fn index() -> &'static str {
    "Welcome to the Linked List Web App!"
}

#[get("/size")]
fn size(state: &rocket::State<AppState>) -> Json<usize> {
    let list = state.list.lock().unwrap();
    Json(list.size())
}

#[derive(Deserialize)]
struct AddData {
    data: i32,
}

#[post("/add", data = "<data>")]
fn add(data: Json<AddData>, state: &rocket::State<AppState>) -> Json<String> {
    let mut list = state.list.lock().unwrap();
    list.add(data.data);
    Json(format!("Added: {}", data.data))
}

#[get("/search?<key>")]
fn search(key: i32, state: &rocket::State<AppState>) -> status::Custom<Json<String>> {
    let list = state.list.lock().unwrap();
    if let Some(node) = list.search(key) {
        status::Custom(Status::Ok, Json(format!("Found: {}", node.data)))
    } else {
        status::Custom(Status::NotFound, Json(format!("Key {} not found", key)))
    }
}

#[derive(Deserialize)]
struct InsertData {
    data: i32,
    index: usize,
}

#[post("/insert", data = "<data>")]
fn insert(data: Json<InsertData>, state: &rocket::State<AppState>) -> status::Custom<Json<String>> {
    let mut list = state.list.lock().unwrap();
    match list.insert(data.data, data.index) {
        Ok(_) => status::Custom(Status::Ok, Json(format!("Inserted: {} at index {}", data.data, data.index))),
        Err(e) => status::Custom(Status::BadRequest, Json(e)),
    }
}

#[derive(Deserialize)]
struct RemoveData {
    key: i32,
}

#[post("/remove", data = "<data>")]
fn remove(data: Json<RemoveData>, state: &rocket::State<AppState>) -> status::Custom<Json<String>> {
    let mut list = state.list.lock().unwrap();
    if list.remove(data.key) {
        status::Custom(Status::Ok, Json(format!("Removed: {}", data.key)))
    } else {
        status::Custom(Status::NotFound, Json(format!("Key {} not found", data.key)))
    }
}

// Launch the Rocket server
#[launch]
fn rocket() -> _ {
    let linked_list = LinkedList::new();
    rocket::build()
        .manage(AppState {
            list: Mutex::new(linked_list),
        })
        .attach(CORS) // Attach CORS Fairing
        .mount("/", routes![index, size, add, search, insert, remove, options]) // Mount `OPTIONS`
}

