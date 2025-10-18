CREATE TABLE IF NOT EXISTS program (
    id      INTEGER NOT NULL,
    program_name   TEXT NOT NULL,
    genre   TEXT NULL,  -- Option<String>
    logo_link   TEXT NULL,  -- Option<String>
    link   TEXT NULL,  -- Option<String>
    synopsis   TEXT NULL,  -- Option<String>

    PRIMARY KEY (id)
);