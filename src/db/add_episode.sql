INSERT INTO episode (program_id, program_title, episode_id, episode_title, suspend_flg, schedule, period, rebroadcast_flg, bilingual_flg, english_flg)
VALUES ((?1), (?2), (?3), (?4), (?5), (?6), (?7), (?8), (?9), (?10))
ON CONFLICT(program_id, episode_id, schedule) DO UPDATE SET
    program_title = COALESCE(excluded.program_title, episode.program_title),
    episode_title = COALESCE(excluded.episode_title, episode.episode_title),
    suspend_flg = excluded.suspend_flg,
    period = excluded.period,
    rebroadcast_flg = excluded.rebroadcast_flg,
    bilingual_flg = excluded.bilingual_flg,
    english_flg = excluded.english_flg;

