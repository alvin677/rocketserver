use rocket::fs::FileServer;
mod api;

#[macro_use] extern crate rocket;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    // HTTP config
    let http_config = rocket::Config::figment()
        .merge(("port", 8000))  // HTTP Port
        .merge(("address", "0.0.0.0"));

    // HTTPS config
    let https_config = rocket::Config::figment()
        .merge(("port", 443))  // HTTPS Port
        .merge(("address", "0.0.0.0"))
        .merge(("tls.certs", "./secret/fullchain.pem"))
        .merge(("tls.key", "./secret/privkey.pem"));

    // HTTP Rocket instance
    let http_server = rocket::custom(http_config)
        .mount("/", routes![api::greet::hello, api::rpi::get_temp])
        .mount("/", FileServer::new("./html", rocket::fs::Options::Index));

    // HTTPS Rocket instance
    let https_server = rocket::custom(https_config)
        .mount("/", routes![api::greet::hello, api::rpi::get_temp])
        .mount("/", FileServer::new("./html", rocket::fs::Options::Index));

    // Run both servers simultaneously
    rocket::tokio::join!(http_server.launch(), https_server.launch());

    Ok(())
}
