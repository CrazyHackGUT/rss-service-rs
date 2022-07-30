BEGIN;

LOCK TABLE webhooks_subscriptions IN EXCLUSIVE MODE;
SELECT setval('webhooks_subscriptions_id_seq', COALESCE((SELECT MAX(id)+1 FROM webhooks_subscriptions), 1), false);

COMMIT;
