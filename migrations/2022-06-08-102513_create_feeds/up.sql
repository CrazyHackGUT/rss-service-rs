create table feeds
(
    id bigserial
        constraint feed_pk primary key,
    url        varchar(255) NOT NULL,
    home_page  varchar(255) NOT NULL,
    title      varchar(128) NOT NULL,
    updated_at timestamp NOT NULL
);

create unique index feeds_url_uindex
    on feeds (url);
