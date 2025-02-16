-- Add migration script here
CREATE TABLE IF NOT EXISTS "sessions"  (
  "session_id" uuid PRIMARY KEY NOT NULL,
  "user_id" uuid NOT NULL,
  "created_at" TIMESTAMP DEFAULT 'now()'
);

ALTER TABLE "sessions" ADD FOREIGN KEY ("user_id") REFERENCES "users" ("user_id");

CREATE OR REPLACE FUNCTION remove_expired_sessions()
    RETURNS TRIGGER 
    LANGUAGE plpgsql
AS $$
BEGIN
    DELETE FROM sessions WHERE created_at < NOW() - INTERVAL '14 day';
    RETURN NEW;
END;
$$;

CREATE TRIGGER expired_sessions 
AFTER INSERT ON "sessions"
FOR EACH ROW 
EXECUTE FUNCTION remove_expired_sessions();

