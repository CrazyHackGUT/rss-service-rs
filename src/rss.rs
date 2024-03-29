use std::thread;
use std::time::Duration;
use chrono::Utc;
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl};
use log::{debug, error, trace};
use rss::Item;
use crate::db::DbConnection;
use crate::handlers::{Message, SharedVector};
use crate::models::dto::webhook_message::WebHookMessage;
use crate::models::feed::Feed;
use crate::models::post::NewPost;
use crate::models::webhook::WebHook;
use crate::models::webhook_subscription::WebHookSubscription;

pub(crate) fn start(vector: SharedVector<Message>) {
    trace!("rss::start()");

    debug!("Spawning a RSS fetcher thread...");
    thread::spawn(move || {
        trace!("rss::start()->closure");

        debug!("Spawned a RSS fetcher thread");
        loop {
            trace!("rss::start()->closure->loop");
            handle_new_rss_posts(&vector); // vector.lock().unwrap().push(handlers::Message {body: "".to_string(), endpoint: "".to_string()});

            thread::sleep(Duration::from_secs(150));
        }
    });
}

fn handle_new_rss_posts(vector: &SharedVector<Message>) {
    use crate::schema::feeds::updated_at;
    trace!("rss::handle_new_rss_posts()");

    let current_ts = Utc::now().naive_utc() - chrono::Duration::seconds(300);

    debug!("Spawning a connection...");
    let connection = crate::db::connection();
    debug!("Fetching feeds for reading...");

    let feeds = crate::schema::feeds::dsl::feeds.filter(updated_at.le(current_ts))
        .load::<Feed>(&connection).unwrap();

    debug!("Fetched {} feeds", feeds.len());
    for feed in feeds {
        handle_feed(&feed, &connection, vector);
    }
}

fn handle_feed(feed: &Feed, connection: &DbConnection, vector: &SharedVector<Message>) {
    trace!("rss::handle_feed()");
    match feed.posts() {
        Ok(items) => {
            handle_non_existing_posts(items, &feed, connection, vector);
            update_feed_fetch(&feed, connection);
        },

        Err(e) => error!("Failed to work with {}: {:?}", feed, e)
    }
}

fn handle_non_existing_posts(items: Vec<Item>, feed: &Feed, connection: &DbConnection, vector: &SharedVector<Message>) {
    use crate::schema::posts::dsl::posts;
    use crate::schema::posts::{feed_id, url};
    trace!("rss::handle_non_existing_posts()");

    debug!("Handling non-existing posts for {}...", feed);
    for item in items {
        let item_ref = &item;
        if item_ref.link.is_none()
        {
            debug!("RSS item doesn't have link. Something wrong. Skipping...");
            continue;
        }

        // TODO: move this shit to repository method?
        debug!("Searching post {} on forum...", item_ref.title.as_ref().unwrap_or(&"Unknown title".to_string()));
        let count = posts.filter(url.eq(item_ref.link.as_ref().unwrap().as_str())).filter(feed_id.eq(feed.id)).count()
            .first::<i64>(connection).unwrap();
        // TODO: move this shit to repository method?

        match count {
            0 => handle_non_existing_post(item_ref, feed, connection, vector),
            1 => debug!("Post already present in database; skipping"),
            _ => todo!()
        };
    }
}

fn handle_non_existing_post(item: &Item, feed: &Feed, connection: &DbConnection, vector: &SharedVector<Message>) {
    use crate::schema::posts::dsl::posts;

    trace!("rss::handle_non_existing_post()");
    let feed_id = feed.id;
    let title = item.title.as_ref().unwrap().to_string();
    let url = item.link.as_ref().unwrap().to_string();
    let posted_at = crate::util::safe_date_from_post(item);
    let received_at = Utc::now().naive_utc();
    debug!("Inserting a new post (feed {}, item link {}) into database", feed_id, url);

    match diesel::insert_into(posts)
        .values(&NewPost {
            feed_id,
            title,
            url,
            posted_at,
            received_at
        }).execute(connection) {
        Ok(_) => {
            debug!("Successfully inserted post {} ({}) in database", item.link.as_ref().unwrap(), feed);
            deliver_post(item, feed, connection, vector);
        },
        Err(_) => debug!("Error caused when inserting post {} ({}) in database", item.link.as_ref().unwrap(), feed)
    };
}

fn update_feed_fetch(feed: &Feed, connection: &DbConnection) {
    use crate::schema::feeds::dsl::feeds;
    use crate::schema::feeds::{updated_at, id};

    trace!("rss::update_feed_fetch({})", feed.id);
    let feeds_filtered = feeds.filter(id.eq(feed.id));
    match diesel::update(feeds_filtered)
        .set(updated_at.eq(Utc::now().naive_utc()))
        .execute(connection) {
        Ok(_) => debug!("Successfully updated `updated_at` for {}", feed),
        Err(_) => debug!("Error caused when updating `updated_at` for {}", feed)
    };
}

fn deliver_post(item: &Item, feed: &Feed, connection: &DbConnection, vector: &SharedVector<Message>) {
    use crate::schema::webhooks_subscriptions::{feed_id, dsl::webhooks_subscriptions};
    use crate::schema::webhooks::dsl::webhooks;

    trace!("rss::deliver_post()");
    let subscribers = webhooks_subscriptions.filter(feed_id.eq(feed.id))
        .inner_join(webhooks)
        .load::<(WebHookSubscription, WebHook)>(connection).unwrap();

    let message = serde_json::to_string(&WebHookMessage::from_item(item, feed))
        .unwrap();

    let mut locked_vector = vector.lock().unwrap();
    for subscriber in subscribers {
        locked_vector.push(Message {
            body: message.to_owned(),
            endpoint: subscriber.1.url
        })
    }
}
