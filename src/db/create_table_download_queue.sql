CREATE TABLE IF NOT EXISTS download_queue (
    program_id      INTEGER NOT NULL,
    episode_id      INTEGER NOT NULL,
    schedule        DATETIME NOT NULL,
    PRIMARY KEY (program_id, episode_id, schedule)
);