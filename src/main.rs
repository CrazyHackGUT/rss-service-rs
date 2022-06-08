#[macro_use]
extern crate diesel;
extern crate log;

use actix_web::{App, HttpServer, web};
use dotenv::dotenv;
use std::env::var;
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use log::info;
use r2d2::Pool;
use reqwest::Client;

mod controllers;
mod models;
mod schema;

type DbPool = Pool<ConnectionManager<PgConnection>>;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    pretty_env_logger::init();

    let web_addr = var("ACTIX_BINDADDR").unwrap_or("127.0.0.1".to_owned());
    let web_port =
        u16::from_str_radix(var("ACTIX_BINDPORT").unwrap_or_default().as_str(), 10).unwrap_or(8080);

    info!("Starting web-server at {}:{}", web_addr, web_port);

    let pool = pool();
    HttpServer::new(move || {
        App::new().configure(controllers::config)
            .app_data(web::Data::new(pool.clone()))
    })
        .bind((web_addr, web_port))
        .unwrap()
        .run()
        .await
}

fn pool() -> DbPool {
    let database_url = var("DATABASE_URL")
        .expect("DATABASE_URL is not set in environment variables");

    Pool::builder()
        .build(ConnectionManager::new(database_url))
        .unwrap()
}

pub(crate) fn http_client() -> Client {
    reqwest::ClientBuilder::new()
        .user_agent("RSS Client Service/1.0".to_owned())
        .build().unwrap()
}
