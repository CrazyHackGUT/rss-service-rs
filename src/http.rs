use std::env::var;
use actix_web::{App, HttpServer, web};
use log::info;

pub(crate) async fn http_server() -> std::io::Result<()> {
    let bindaddr = var("RSS_BINDADDR").unwrap_or("127.0.0.1:8080".to_owned());
    info!("Starting web-server at {}", bindaddr);

    HttpServer::new(move || {
        App::new().configure(crate::controllers::config)
            .app_data(web::Data::new(crate::db::pool()))
    })
        .bind(bindaddr)
        .unwrap()
        .run()
        .await
}