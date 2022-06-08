use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};
use diesel::Queryable;
use crate::schema::*;

#[derive(Deserialize, Serialize)]
pub(crate) struct FeedDto {
    pub url: String,
    pub home_page: Option<String>,
    pub title: Option<String>
}

#[derive(Deserialize, Serialize, Queryable)]
pub(crate) struct Feed {
    pub id: i64,
    pub url: String,

    // HomePage and title can be dynamically set via Feed URL.
    pub home_page: String,
    pub title: String,

    pub updated_at: NaiveDateTime
}

#[derive(Insertable)]
#[table_name="feeds"]
pub(crate) struct NewFeed {
    pub url: String,
    pub home_page: String,
    pub title: String,
    pub updated_at: NaiveDateTime
}
