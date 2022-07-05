create table webhooks (
    id bigint NOT NULL constraint webhooks_pk
                               primary key,
    url varchar(255) NOT NULL
);

CREATE UNIQUE INDEX rss_webhooks_url ON webhooks USING btree (url);
