use serde::{Deserialize, Serialize};
use diesel::Queryable;
use crate::schema::*;

#[derive(Deserialize, Serialize, Queryable)]
pub(crate) struct WebHook {
    pub id: i64,
    pub url: String
}

#[derive(Insertable)]
#[table_name="webhooks"]
pub(crate) struct NewWebHook {
    pub url: String
}
