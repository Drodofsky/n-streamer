DELETE FROM download_queue
WHERE program_id = (?1)
AND episode_id = (?2)
AND schedule = (?3);