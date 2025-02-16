-- Add migration script here
CREATE TABLE "challenges" IF NOT EXISTS (
  "challenge_id" uuid PRIMARY KEY NOT NULL,
  "challenger_id" uuid NOT NULL,
  "opponent_id" uuid NOT NULL,
  "status" status NOT NULL,
  "created_at" TIMESTAMP DEFAULT 'now()'
);

ALTER TABLE "challenges" ADD FOREIGN KEY ("challenger_id") REFERENCES "users" ("user_id");

ALTER TABLE "challenges" ADD FOREIGN KEY ("opponent_id") REFERENCES "users" ("user_id");

CREATE OR REPLACE FUNCTION remove_expired_challenges()
    RETURNS TRIGGER 
    LANGUAGE plpgsql
AS $$
BEGIN
    DELETE FROM challenges WHERE status = 'pending' AND created_at < NOW() - INTERVAL '1 day';
    DELETE FROM challenges WHERE created_at > NOW() - INTERVAL '1 day';
    RETURN NEW;
END;
$$;

CREATE TRIGGER expired_challenge
AFTER INSERT ON "challenges"
FOR EACH ROW 
EXECUTE FUNCTION remove_expired_challenges();

