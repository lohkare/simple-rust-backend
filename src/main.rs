#[macro_use]
extern crate lazy_static;
use actix_web::{App, HttpServer};
use crate::configuration::Configuration;

mod go_rest_client;
mod configuration;
mod user_service;

lazy_static! {
    static ref CONFIG: Configuration = configuration::Configuration::read_from_config_file("resources/config").unwrap();
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    println!("Starting simple-backend -system...");
    println!();
    HttpServer::new(|| {
        App::new()
            .service(user_service::hello)
            .service(user_service::get_users)
            .service(user_service::get_user)
    })
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
