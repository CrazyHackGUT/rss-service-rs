use super::super::models::feed::Feed;
use actix_web::{get, post, web, HttpResponse, Responder};

#[get("/v1/feed/")]
pub(super) async fn list() -> impl Responder {
    HttpResponse::Ok()
}

#[get("/v1/feed/{feed_id}")]
pub(super) async fn get(feed_id: web::Path<u64>) -> impl Responder {
    HttpResponse::Ok()
}

#[post("/v1/feed/")]
pub(super) async fn new(feed: web::Json<Feed>) -> impl Responder {
    HttpResponse::Ok()
}
