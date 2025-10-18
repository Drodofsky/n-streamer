INSERT INTO program (id, program_name, genre, logo_link, link, synopsis)
VALUES ((?1), (?2), (?3), (?4), (?5), (?6))
ON CONFLICT(id) DO UPDATE SET
    program_name = COALESCE(excluded.program_name, program.program_name),
    genre = COALESCE(excluded.genre, program.genre),
    logo_link = COALESCE(excluded.logo_link, program.logo_link),
    link = COALESCE(excluded.link, program.link),
    synopsis = COALESCE(excluded.synopsis, program.synopsis);