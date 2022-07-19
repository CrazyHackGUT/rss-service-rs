use chrono::{DateTime, NaiveDateTime};
use rss::Item;
use serde::{Deserialize, Serialize};
use crate::models::feed::Feed;
use crate::models::post::Post;

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
    pub(crate) fn from_post(post: &Post) -> Self {
        PostStruct {
            title: post.title.to_owned(),
            url: post.url.to_owned(),
            posted_at: post.posted_at
        }
    }

    pub(crate) fn from_item(item: &Item) -> Self {
        PostStruct {
            title: item.title.as_ref().unwrap().to_owned(),
            url: item.link.as_ref().unwrap().to_owned(),
            posted_at: DateTime::parse_from_rfc2822(item.pub_date.as_ref().unwrap().as_str())
                .unwrap().naive_utc()
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
    pub(crate) fn from_post(post: &Post, feed: &Feed) -> Self {
        WebHookMessage {
            post: PostStruct::from_post(post),
            feed: FeedStruct::from(feed)
        }
    }

    pub(crate) fn from_item(item: &Item, feed: &Feed) -> Self {
        WebHookMessage {
            post: PostStruct::from_item(item),
            feed: FeedStruct::from(feed)
        }
    }
}
