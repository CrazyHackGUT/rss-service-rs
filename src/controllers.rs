use actix_web::web::ServiceConfig;

mod feeds;

pub(crate) fn config(cfg: &mut ServiceConfig) {
    cfg.service(feeds::new)
        .service(feeds::get)
        .service(feeds::list);
}
