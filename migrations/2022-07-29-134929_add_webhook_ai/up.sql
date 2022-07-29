-- Create a Sequence
CREATE SEQUENCE webhook_id_seq;

-- Use it to provide a new value for each project ID
ALTER TABLE webhooks
    ALTER id
        SET DEFAULT NEXTVAL('webhook_id_seq');