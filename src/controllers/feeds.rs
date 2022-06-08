use actix_web::{get, HttpResponse, post, Responder, web};
use super::super::models::feed::Feed;

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
