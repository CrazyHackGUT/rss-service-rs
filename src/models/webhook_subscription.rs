use serde::{Deserialize, Serialize};
use diesel::Queryable;
use crate::schema::*;

#[derive(Deserialize, Serialize, Queryable)]
pub(crate) struct WebHookSubscription {
    pub id: i64,
    pub webhook_id: i64,
    pub feed_id: i64,
}

#[derive(Insertable)]
#[table_name="webhooks_subscriptions"]
pub(crate) struct NewWebHook {
    pub webhook_id: i64,
    pub feed_id: i64,
}
