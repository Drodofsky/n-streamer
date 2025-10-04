CREATE TABLE IF NOT EXISTS episode (
    program_id      INTEGER NOT NULL,
    program_title   TEXT NOT NULL,
    episode_id      INTEGER NOT NULL,
    episode_title   TEXT NULL,  -- Option<String>

    suspend_flg     INTEGER NOT NULL CHECK (suspend_flg IN (0,1)),

    schedule        DATETIME NOT NULL,
    period          INTEGER NOT NULL, -- store seconds

    rebroadcast_flg INTEGER NULL CHECK (rebroadcast_flg IN (0,1)), -- Option<bool>
    bilingual_flg   INTEGER NULL CHECK (bilingual_flg IN (0,1)),   -- Option<bool>
    english_flg     INTEGER NULL CHECK (english_flg IN (0,1)),     -- Option<bool>

    PRIMARY KEY (program_id, episode_id, schedule)
);