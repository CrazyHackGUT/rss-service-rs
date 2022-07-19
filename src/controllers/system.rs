use actix_web::{Responder, web, get, HttpResponse};
use diesel::{QueryDsl, RunQueryDsl};
use serde_json::json;
use crate::db::DbPool;

#[get("/v1/health")]
pub(super) async fn health(pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::{feeds::dsl::feeds, posts::dsl::posts,
        webhooks::dsl::webhooks, webhooks_subscriptions::dsl::webhooks_subscriptions};

    match super::db_from_pool(pool) {
        Err(e) => e,
        Ok(db) => HttpResponse::Ok().json(json!({
            "success": true,
            "stats": {
                "feeds": feeds.count().first::<i64>(&db).unwrap(),
                "posts": posts.count().first::<i64>(&db).unwrap(),
                "subscribers": webhooks.count().first::<i64>(&db).unwrap(),
                "subscriptions": webhooks_subscriptions.count().first::<i64>(&db).unwrap()
            }
        }))
    }
}
