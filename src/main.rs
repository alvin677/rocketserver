use rocket::fs::{FileServer, relative};
use std::fs;

#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> rocket::response::content::RawHtml<String> {
    let index_file = fs::read_to_string("./static/index.html").unwrap();
    rocket::response::content::RawHtml(index_file)
}

#[get("/hello/<name>/<age>")]
fn hello(name: &str, age: u8) -> String {
    format!("Hello, {} year old named {}!", age, name)
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, hello])
        .mount("/static", FileServer::from(relative!("static")))
}
