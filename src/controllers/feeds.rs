use super::super::models::feed::{Feed, NewFeed};
use actix_web::{get, post, web, HttpResponse, Responder};
use chrono::NaiveDateTime;
use diesel::prelude::*;
use reqwest::Url;
use rss::Channel;
use serde_json::json;
use crate::db::DbPool;
use crate::models::feed::FeedDto;

use crate::http_client;

use crate::schema::feeds::dsl::*;
use super::db_from_pool;

#[get("/v1/feed/")]
pub(super) async fn list(pool: web::Data<DbPool>) -> impl Responder {
    match db_from_pool(pool) {
        Err(e) => e,
        Ok(db) => HttpResponse::Ok()
            .json(feeds.load::<Feed>(&db).unwrap())
    }
}

#[get("/v1/feed/{feed_id}")]
pub(super) async fn get(feed_id: web::Path<i64>, pool: web::Data<DbPool>) -> impl Responder {
    match db_from_pool(pool) {
        Err(e) => e,
        Ok(db) => {
            let find_result = feeds.find(feed_id.into_inner()).load::<Feed>(&db);
            find_result.map(|feeds_vec| {
                feeds_vec.first().map(|feed| HttpResponse::Ok().json(&feed))
                    .unwrap_or_else(|| HttpResponse::NotFound().json(json!({
                        "success": false,
                        "code": "not_found"
                    })))
            }).unwrap_or_else(|_| HttpResponse::InternalServerError().json(json!({
                    "success": false,
                    "code": "database_failure"
                })))
        }
    }
}

#[post("/v1/feed/")]
pub(super) async fn new(feed: web::Json<FeedDto>, pool: web::Data<DbPool>) -> impl Responder {
    let mut feed = feed.into_inner();
    let home_is_filled = feed.home_page.is_some();
    let title_is_filled = feed.title.is_some();

    let feed_url = Url::parse(feed.url.as_str());
    if feed_url.is_err() {
        return HttpResponse::BadRequest().json(json!({
            "success": false,
            "code": "bad_url"
        }));
    }

    let feed_url = feed_url.unwrap();

    // Request RSS feed only if home page or title is not filled.
    if !(home_is_filled || title_is_filled) {
        let http_result = http_client().get(feed_url.to_owned()).send().await;
        if http_result.is_err() {
            let err = http_result.err().unwrap();
            return HttpResponse::NotAcceptable().json(json!({
                "success": false,
                "code": "homepage_title_not_set",
                "reason": format!("Can't fetch HomePage URL and Title: {}. Please, set it manually in request parameters.", err)
            }));
        }

        let body = &http_result.unwrap().bytes().await.unwrap()[..];
        let channel_read_result = Channel::read_from(body);
        if channel_read_result.is_err() {
            let err = channel_read_result.err().unwrap();
            return HttpResponse::NotAcceptable().json(json!({
                "success": false,
                "code": "unexpected_response",
                "reason": format!("Can't fetch HomePage URL and Title: unexpected response ({}). Please, set it manually in request parameters.", err)
            }));
        }

        let channel = channel_read_result.unwrap();

        if !title_is_filled {
            feed.title = Some(channel.title);
        }

        if !home_is_filled {
            feed.home_page = Some(channel.link);
        }
    }

    match db_from_pool(pool) {
        Err(e) => e,
        Ok(db) => {
            let feed = NewFeed {
                url: feed_url.to_string(),
                home_page: feed.home_page.unwrap(),
                title: feed.title.unwrap(),
                updated_at: NaiveDateTime::from_timestamp(0, 0)
            };

            diesel::insert_into(feeds)
                .values(feed)
                .get_result::<Feed>(&db)
                .map(|f| HttpResponse::Ok().json(&f))
                .unwrap_or_else(|_| HttpResponse::InternalServerError().json(json!({
                    "success": false,
                    "code": "database_failure"
                })))
        }
    }
}
