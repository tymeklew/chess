-- Add migration script here
ALTER TABLE friend_requests
ALTER COLUMN status SET DEFAULT 'pending',
ALTER COLUMN status SET NOT NULL;