use std::fmt::{Display, Formatter};
use crate::err::Error;
use chrono::{NaiveDateTime};
use serde::{Deserialize, Serialize};
use diesel::Queryable;
use rss::Channel;
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

impl Feed {
    pub(crate) fn posts(self: &Self) -> Result<Vec<rss::Item>, Error> {
        let response = reqwest::blocking::get(self.url.to_owned())
            .map_err(Error::ReqwestError)?;

        let body = &response.bytes()
            .map_err(Error::ReqwestError)?[..];

        let channel = Channel::read_from(body)
            .map_err(Error::RssError)?;

        Ok(channel.items)
    }
}

impl Display for Feed {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{} ({})", self.title, self.url).as_str())
    }
}

#[derive(Insertable)]
#[table_name="feeds"]
pub(crate) struct NewFeed {
    pub url: String,
    pub home_page: String,
    pub title: String,
    pub updated_at: NaiveDateTime
}
