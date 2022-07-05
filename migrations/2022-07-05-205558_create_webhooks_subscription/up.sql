create table webhooks_subscriptions
(
    id bigserial constraint webhooks_subscriptions_pk
        primary key,
    webhook_id bigint not null
        constraint webhooks_subscriptions_webhooks_id_fk
            references webhooks
            on update cascade on delete cascade,
    feed_id bigint not null
        constraint webhooks_subscriptions_feeds_id_fk
            references feeds
            on update cascade on delete cascade
);

CREATE UNIQUE INDEX rss_webhooks_subscriptions_webhook_feed ON webhooks_subscriptions USING btree (webhook_id, feed_id);
