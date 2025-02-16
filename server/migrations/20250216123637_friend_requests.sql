-- Add migration script here
ALTER TABLE "friend_requests" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("user_id");

ALTER TABLE "friend_requests" ADD FOREIGN KEY ("friend_id") REFERENCES "users" ("user_id");