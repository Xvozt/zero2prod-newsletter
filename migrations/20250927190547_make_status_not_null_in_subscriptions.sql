-- Add migration script here
BEGIN;
    UPDATE subscriptions
        SET status = 'confirmed'
        WHERE STATUS IS NULL;
    ALTER TABLE subscriptions ALTER COLUMN status SET NOT NULL;