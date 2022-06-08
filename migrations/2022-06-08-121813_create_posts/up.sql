create table posts
(
    id bigserial constraint posts_pk
        primary key,
    feed_id bigint not null
        constraint posts_feeds_id_fk
            references feeds
            on update cascade on delete cascade,
    title varchar(255) not null,
    url varchar(255) not null,
    posted_at timestamp not null,
    received_at timestamp not null
);
