#[macro_use]
extern crate diesel;
extern crate log;

use dotenv::dotenv;
use reqwest::Client;

mod err;
mod controllers;
mod models;
mod schema;
mod http;
mod db;
mod rss;
mod handlers;
mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    pretty_env_logger::init();

    let vector = handlers::start();
    rss::start(vector);
    http::http_server()
        .await
}

pub(crate) fn http_client() -> Client {
    reqwest::ClientBuilder::new()
        .user_agent("RSS Client Service/1.0".to_owned())
        .build().unwrap()
}
