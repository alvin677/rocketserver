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
#[get("/<file..>")]
fn site(file: std::path::PathBuf) -> rocket::response::content::RawHtml<String> {
    let mut index_file: String = fs::read_to_string("./html/err.html").unwrap();
    let path = format!("./html/{}", file.to_str().unwrap());
    
    if fs::metadata(path.to_owned()+"/index.html").is_ok() {
        index_file = fs::read_to_string(path+"/index.html").unwrap();
    } else if fs::metadata(path.to_owned()).is_ok() {
        index_file = fs::read_to_string(&path).unwrap();
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
