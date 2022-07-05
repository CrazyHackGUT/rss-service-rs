CREATE UNIQUE INDEX rss_post_feed_url ON posts USING btree (feed_id, url);
