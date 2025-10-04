INSERT INTO episode (program_id, program_title, episode_id, episode_title, suspend_flg, schedule, period, rebroadcast_flg, bilingual_flg, english_flg)
VALUES ((?1), (?2), (?3), (?4), (?5), (?6), (?7), (?8), (?9), (?10))
ON CONFLICT(program_id, episode_id, schedule) DO NOTHING;