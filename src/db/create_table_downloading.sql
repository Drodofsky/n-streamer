CREATE TABLE IF NOT EXISTS downloading (
    program_id      INTEGER NOT NULL,
    episode_id      INTEGER NOT NULL,
    schedule        DATETIME NOT NULL,
    PRIMARY KEY (program_id, episode_id, schedule)
);