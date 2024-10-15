use rocket::fs::{FileServer, relative};
use std::fs;
mod api;

#[macro_use] extern crate rocket;

// index.html file in directory
#[get("/")]
fn index() -> rocket::response::content::RawHtml<String> {
    return rocket::response::content::RawHtml(fs::read_to_string("./html/index.html").unwrap());
}

// any other file in the directory
#[get("/<file>")]
fn site(file: &str) -> rocket::response::content::RawHtml<String> {
    let index_file: String;
    if fs::exists(format!("./html/{}.html", file)).unwrap() {
        index_file = fs::read_to_string(format!("./html/{}.html", file)).unwrap();
    } else {
        index_file = fs::read_to_string("./html/err.html").unwrap();
    }
    return rocket::response::content::RawHtml(index_file);
}

// define routes to use
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index, api::greet::hello, site, api::rpi::get_temp])
        .mount("/html", FileServer::from(relative!("html")))
}
