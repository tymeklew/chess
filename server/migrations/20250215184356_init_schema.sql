CREATE TYPE status AS ENUM ('pending', 'accepted', 'rejected' , 'rejected');
CREATE TABLE "users" IF NOT EXISTS (
  "user_id" uuid PRIMARY KEY NOT NULL,
  "email" varchar(320) NOT NULL,
  "username" varchar(20) NOT NULL,
  "password" varchar NOT NULL,
  "created_at" TIMESTAMP DEFAULT 'now()'
);

CREATE TABLE "friendships" IF NOT EXISTS (
  "user_id" uuid NOT NULL,
  "friend_id" uuid NOT NULL,
  PRIMARY KEY ("user_id", "friend_id")
);

CREATE TABLE "friend_requests" IF NOT EXISTS (
  "request_id" uuid PRIMARY KEY NOT NULL,
  "user_id" uuid NOT NULL,
  "friend_id" uuid NOT NULL,
  "status" status,
  "created_at" timestamp NOT NULL DEFAULT 'now()'
);

CREATE TABLE "games" IF NOT EXISTS (
  "game_id" uuid PRIMARY KEY NOT NULL,
  "white_id" uuid NOT NULL,
  "black_id" uuid NOT NULL,
  "started" TIMESTAMP NOT NULL,
  "finished" TIMESTAMP,
  "pgn" TEXT NOT NULL,
  "winner" uuid
);

CREATE TABLE "chat" IF NOT EXISTS (
  "chat_id" uuid PRIMARY KEY NOT NULL,
  "game_id" uuid NOT NULL,
  "sender_id" uuid NOT NULL,
  "message" TEXT NOT NULL,
  "sent_at" TIMESTAMP DEFAULT 'now()'
);

CREATE UNIQUE INDEX ON "users" ("email");

CREATE UNIQUE INDEX ON "users" ("username");

CREATE INDEX ON "friend_requests" ("user_id", "friend_id");

CREATE INDEX ON "chat" ("game_id");

ALTER TABLE "friendships" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("user_id");

ALTER TABLE "friendships" ADD FOREIGN KEY ("friend_id") REFERENCES "users" ("user_id");

ALTER TABLE "games" ADD FOREIGN KEY ("white_id") REFERENCES "users" ("user_id");

ALTER TABLE "games" ADD FOREIGN KEY ("black_id") REFERENCES "users" ("user_id");

ALTER TABLE "games" ADD FOREIGN KEY ("winner") REFERENCES "users" ("user_id");

ALTER TABLE "chat" ADD FOREIGN KEY ("sender_id") REFERENCES "users" ("user_id");

ALTER TABLE "chat" ADD FOREIGN KEY ("game_id") REFERENCES "games" ("game_id");
