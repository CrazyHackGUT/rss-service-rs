use chrono::{DateTime, NaiveDateTime, Utc};
use log::trace;
use rss::Item;

pub(crate) fn safe_date_from_post(item: &Item) -> NaiveDateTime {
    trace!("util::safe_date_from_post()");

    match item.pub_date.as_ref() {
        Some(date) => DateTime::parse_from_rfc2822(date.as_str()).unwrap().naive_utc(),
        None => Utc::now().naive_utc()
    }
}
