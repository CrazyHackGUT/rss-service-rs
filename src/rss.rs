use std::thread;
use std::time::Duration;
use chrono::{DateTime, Utc};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use log::{debug, error, info};
use rss::Item;
use crate::db::DbConnection;
use crate::models::feed::Feed;
use crate::models::post::NewPost;

pub(crate) fn start() {
    debug!("Spawning a RSS fetcher thread...");
    thread::spawn(|| {
        info!("Spawned a RSS fetcher thread");
        loop {
            handle_new_rss_posts();

            thread::sleep(Duration::from_secs(150));
        }
    });
}

fn handle_new_rss_posts() {
    use crate::schema::feeds::updated_at;

    let current_ts = Utc::now().naive_utc() - chrono::Duration::seconds(300);

    debug!("Spawning a connection...");
    let connection = crate::db::connection();
    debug!("Fetching feeds for reading...");

    let feeds = crate::schema::feeds::dsl::feeds.filter(updated_at.le(current_ts))
        .load::<Feed>(&connection).unwrap();

    debug!("Fetched {} feeds", feeds.len());
    for feed in feeds {
        handle_feed(&feed, &connection);
    }
}

fn handle_feed(feed: &Feed, connection: &DbConnection) {
    match feed.posts() {
        Ok(items) => {
            handle_non_existing_posts(items, &feed, connection);
            update_feed_fetch(&feed, connection);
        },

        Err(e) => error!("Failed to work with {}: {:?}", feed, e)
    }
}

fn handle_non_existing_posts(items: Vec<Item>, feed: &Feed, connection: &DbConnection) {
    use crate::schema::posts::dsl::posts;
    use crate::schema::posts::{feed_id, url};

    debug!("Handling non-existing posts for {}...", feed);
    for item in items {
        let item_ref = &item;

        debug!("Searching post {} on forum...", item_ref.title.as_ref().unwrap_or(&"Unknown title".to_string()));
        let count = posts.filter(url.eq(item_ref.link.as_ref().unwrap().as_str())).filter(feed_id.eq(feed.id)).count()
            .first::<i64>(connection).unwrap();

        match count {
            0 => handle_non_existing_post(item_ref, feed, connection),
            1 => debug!("Post already present in database; skipping"),
            _ => todo!()
        };
    }
}

fn handle_non_existing_post(item: &Item, feed: &Feed, connection: &DbConnection) {
    use crate::schema::posts::dsl::posts;

    debug!("Inserting a new post (feed {}, item link {}) into database", feed.id, item.link.as_ref().unwrap());
    match diesel::insert_into(posts)
        .values(&NewPost {
            feed_id: feed.id,
            title: item.title.as_ref().unwrap().to_string(),
            url: item.link.as_ref().unwrap().to_string(),
            posted_at: DateTime::parse_from_rfc2822(item.pub_date.as_ref().unwrap().as_str()).unwrap().naive_utc(),
            received_at: Utc::now().naive_utc()
        }).execute(connection) {
        Ok(_) => debug!("Successfully inserted post {} ({}) in database", item.link.as_ref().unwrap(), feed),
        Err(_) => debug!("Error caused when inserting post {} ({}) in database", item.link.as_ref().unwrap(), feed)
    };
}

fn update_feed_fetch(feed: &Feed, connection: &DbConnection) {
    use crate::schema::feeds::dsl::feeds;
    use crate::schema::feeds::{updated_at, id};

    let feeds_filtered = feeds.filter(id.eq(feed.id));
    match diesel::update(feeds_filtered)
        .set(updated_at.eq(Utc::now().naive_utc()))
        .execute(connection) {
        Ok(_) => debug!("Successfully updated `updated_at` for {}", feed),
        Err(_) => debug!("Error caused when updating `updated_at` for {}", feed)
    };
}
