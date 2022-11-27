use chrono::{DateTime, NaiveDateTime};
use rss::Item;
use serde::{Deserialize, Serialize};
use crate::models::feed::Feed;

#[derive(Serialize, Deserialize)]
pub(crate) struct WebHookMessage {
    pub feed: FeedStruct,
    pub post: PostStruct
}

#[derive(Serialize, Deserialize)]
pub(crate) struct FeedStruct {
    pub url: String,
    pub home_page: String,
    pub title: String
}

#[derive(Serialize, Deserialize)]
pub(crate) struct PostStruct {
    pub title: String,
    pub url: String,

    pub posted_at: NaiveDateTime
}

impl PostStruct {
    pub(crate) fn from_item(item: &Item) -> Self {
        PostStruct {
            title: item.title.as_ref().unwrap().to_owned(),
            url: item.link.as_ref().unwrap().to_owned(),
            posted_at: crate::util::safe_date_from_post(item)
        }
    }
}

impl FeedStruct {
    pub(crate) fn from(feed: &Feed) -> Self {
        FeedStruct {
            url: feed.url.to_owned(),
            home_page: feed.home_page.to_owned(),
            title: feed.title.to_owned()
        }
    }
}

impl WebHookMessage {
    pub(crate) fn from_item(item: &Item, feed: &Feed) -> Self {
        WebHookMessage {
            post: PostStruct::from_item(item),
            feed: FeedStruct::from(feed)
        }
    }
}
