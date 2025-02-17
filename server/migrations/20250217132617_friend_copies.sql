-- Add migration script here
ALTER TABLE friend_requests
ADD CONSTRAINT unique_user_friend_request UNIQUE (user_id, friend_id);
