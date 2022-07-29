ALTER TABLE webhooks
    ALTER id
        SET DEFAULT NULL;

DROP SEQUENCE webhook_id_seq;
