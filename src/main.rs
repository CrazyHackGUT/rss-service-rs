use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env::var;

mod controllers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let web_addr = var("ACTIX_BINDADDR").unwrap_or("127.0.0.1".to_owned());
    let web_port =
        u16::from_str_radix(var("ACTIX_BINDPORT").unwrap_or_default().as_str(), 10).unwrap_or(8080);

    HttpServer::new(|| App::new().configure(controllers::config))
        .bind((web_addr, web_port))
        .unwrap()
        .run()
        .await
}
