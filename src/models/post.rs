use std::fmt::{Display, Formatter};
use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};
use diesel::Queryable;
use crate::schema::*;
use super::feed::Feed;

#[derive(Deserialize, Serialize, Associations, Queryable)]
#[belongs_to(Feed)]
pub(crate) struct Post {
    pub id: i64,
    pub feed_id: i64,

    pub title: String,
    pub url: String,

    pub posted_at: NaiveDateTime,
    pub received_at: NaiveDateTime
}

impl Display for Post {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{} ({})", self.title, self.url).as_str())
    }
}

#[derive(Insertable)]
#[table_name="posts"]
pub(crate) struct NewPost {
    pub feed_id: i64,
    pub title: String,
    pub url: String,
    pub posted_at: NaiveDateTime,
    pub received_at: NaiveDateTime
}

