CREATE TABLE IF NOT EXISTS episode (
    program_id      INTEGER NOT NULL,
    program_title   TEXT NOT NULL,
    episode_id      INTEGER NOT NULL,
    episode_title   TEXT NULL,  -- Option<String>

    suspend_flg     INTEGER NOT NULL,

    schedule        DATETIME NOT NULL,
    period          INTEGER NOT NULL, -- store seconds

    rebroadcast_flg INTEGER NULL,
    bilingual_flg   INTEGER NULL, 
    english_flg     INTEGER NULL, 

    PRIMARY KEY (program_id, episode_id, schedule)
);