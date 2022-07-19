use actix_web::{HttpResponse, web};
use actix_web::web::ServiceConfig;
use serde_json::json;
use crate::db::{DbPool, DbPooledConnection};

mod feeds;
mod subscriptions;
mod system;

pub(crate) fn config(cfg: &mut ServiceConfig) {
    cfg.service(feeds::new)
        .service(feeds::get)
        .service(feeds::list)
        .service(system::health)
        .service(subscriptions::register)
        .service(subscriptions::unsubscribe);
}

pub(crate) fn db_from_pool(pool: web::Data<DbPool>) -> Result<DbPooledConnection, HttpResponse> {
    match pool.get() {
        Err(_) => Err(HttpResponse::InternalServerError().json(json!({
            "success": false,
            "code": "database_unavailable"
        }))),

        Ok(db) => Ok(db)
    }
}
