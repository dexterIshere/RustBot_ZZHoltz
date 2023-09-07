-- Add migration script here
-- UP
CREATE TABLE IF NOT EXISTS quiz_scores (
    discord_id INTEGER PRIMARY KEY,
    quiz_score INTEGER
);