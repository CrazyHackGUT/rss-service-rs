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

joinable!(posts -> feeds (feed_id));

allow_tables_to_appear_in_same_query!(
    feeds,
    posts,
);
