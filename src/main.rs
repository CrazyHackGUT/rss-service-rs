use actix_web::{App, HttpServer};

mod controllers;
mod models;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .configure(controllers::config)
    })
        .bind(("127.0.0.1", 8080))
        .unwrap()
        .run()
        .await
}
