use actix_web::{post, delete, web, HttpResponse, Responder};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde_json::json;
use crate::db::{DbPool};
use super::db_from_pool;

#[post("/v1/subscriptions/create")]
pub(super) async fn register(pool: web::Data<DbPool>) -> impl Responder {
    match db_from_pool(pool) {
        Err(response) => response,
        Ok(_db) => {
            // TODO: add insert a subscription
            HttpResponse::Ok().json(json!({
                "success": true
            }))
        }
    }
}

#[delete("/v1/subscriptions/{subscription_id}")]
pub(super) async fn unsubscribe(subscription_id: web::Path<i64>, pool: web::Data<DbPool>) -> impl Responder {
    use crate::schema::webhooks_subscriptions::dsl::{webhooks_subscriptions, id};

    match db_from_pool(pool) {
        Err(response) => response,
        Ok(db) => {
            let delete_statement = diesel::delete(webhooks_subscriptions)
                .filter(id.eq(subscription_id.into_inner()))
                .execute(&db);

            match delete_statement {
                Err(_) => HttpResponse::InternalServerError().json(json!({
                    "success": false,
                    "code": "internal_database_error"
                })),

                Ok(count) => {
                    if count > 0 {
                        HttpResponse::Ok().finish()
                    } else {
                        HttpResponse::BadRequest().finish()
                    }
                }
            }
        }
    }
}

// fn subscriber_by_url(db: DbPooledConnection) -> Option<>
