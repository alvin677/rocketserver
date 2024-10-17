use rocket::fs::FileServer;
use rocket::config::Config;
use rocket::tokio;
mod api;
#[macro_use] extern crate rocket;

#[launch]
async fn rocket() -> _ {
    let https_config = Config::figment()
    .merge(("port", 8000))
    .merge(("address", "0.0.0.0"))
    .merge(("tls.certs", "./secret/fullchain.pem"))
    .merge(("tls.key", "./secret/privkey.pem"));

    let http_config = Config::figment()
    .merge(("port", 8080))
    .merge(("address", "0.0.0.0"));

    let https_app = rocket::custom(https_config)
    .mount("/", routes![api::greet::hello, api::rpi::get_temp])
    .mount("/", FileServer::new("./html", rocket::fs::Options::Index));

    let http_app = rocket::custom(http_config)
    .mount("/", routes![api::greet::hello, api::rpi::get_temp])
    .mount("/", FileServer::new("./html", rocket::fs::Options::Index));

    // Launch HTTP server in a separate task
    tokio::spawn(async move {
        if let Err(e) = http_app.launch().await {
            println!("HTTP server error: {}", e);
        }
    });

    // Return the HTTPS app to be launched by Rocket
    https_app
}
