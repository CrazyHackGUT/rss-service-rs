use actix_web::{post, delete, web, HttpResponse, Responder};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use serde_json::json;
use serde::{Deserialize, Serialize};
use crate::db::{DbPool, DbPooledConnection};
use crate::models::webhook::{NewWebHook, WebHook};
use crate::models::webhook_subscription::{NewWebHookSubscription, WebHookSubscription};
use super::db_from_pool;

#[derive(Serialize, Deserialize)]
pub struct SubscriberRequest {
    webhook_url: String,
    feed_id: i64
}

#[post("/v1/subscriptions/create")]
pub(super) async fn register(pool: web::Data<DbPool>, request: web::Json<SubscriberRequest>) -> impl Responder {
    match db_from_pool(pool) {
        Err(response) => response,
        Ok(db) => {
            let webhook = subscriber_by_url(&db, request.webhook_url.to_owned());
            let subscription = subscription_by_subscriber_feed(&db, webhook, request.feed_id);

            HttpResponse::Ok().json(&subscription)
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

fn subscriber_by_url(db: &DbPooledConnection, webhook_url: String) -> WebHook {
    use crate::schema::webhooks::{url, dsl::webhooks};

    match webhooks.filter(url.eq(webhook_url.to_owned())).first::<WebHook>(db) {
        Ok(webhook) => webhook,
        Err(_) => {
            let webhook = NewWebHook {
                url: webhook_url
            };

            diesel::insert_into(webhooks).values(webhook)
                .get_result::<WebHook>(db).unwrap()
        }
    }
}

fn subscription_by_subscriber_feed(db: &DbPooledConnection, webhook: WebHook, feed: i64) -> WebHookSubscription {
    use crate::schema::webhooks_subscriptions::{feed_id, webhook_id, dsl::webhooks_subscriptions};

    match webhooks_subscriptions.filter(feed_id.eq(feed.to_owned()))
        .filter(webhook_id.eq(webhook.id)).first::<WebHookSubscription>(db) {
        Ok(subscription) => subscription,
        Err(_) => {
            let subscription = NewWebHookSubscription {
                webhook_id: webhook.id,
                feed_id: feed
            };

            diesel::insert_into(webhooks_subscriptions).values(subscription)
                .get_result::<WebHookSubscription>(db).unwrap()
        }
    }
}
