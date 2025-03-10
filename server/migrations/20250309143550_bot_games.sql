-- Add migration script here
CREATE TABLE IF NOT EXISTS "bot_games" (
    id UUID PRIMARY KEY NOT NULL,
    player_id UUID NOT NULL,
    white BOOLEAN NOT NULL,
    difficulty INTEGER NOT NULL,
    png TEXT NOT NULL,
    won BOOLEAN NOT NULL
); 
ALTER TABLE bot_games ADD FOREIGN KEY (player_id) REFERENCES users (user_id);

ALTER TABLE games RENAME TO player_games;