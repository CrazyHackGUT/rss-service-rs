table! {
    feeds (id) {
        id -> Int8,
        url -> Varchar,
        home_page -> Varchar,
        title -> Varchar,
        updated_at -> Timestamp,
    }
}

table! {
    posts (id) {
        id -> Int8,
        feed_id -> Int8,
        title -> Varchar,
        url -> Varchar,
        posted_at -> Timestamp,
        received_at -> Timestamp,
    }
}

table! {
    webhooks (id) {
        id -> Int8,
        url -> Varchar,
    }
}

table! {
    webhooks_subscriptions (id) {
        id -> Int8,
        webhook_id -> Int8,
        feed_id -> Int8,
    }
}

joinable!(posts -> feeds (feed_id));
joinable!(webhooks_subscriptions -> feeds (feed_id));
joinable!(webhooks_subscriptions -> webhooks (webhook_id));

allow_tables_to_appear_in_same_query!(
    feeds,
    posts,
    webhooks,
    webhooks_subscriptions,
);
