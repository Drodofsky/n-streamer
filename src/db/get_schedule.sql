SELECT e.program_id,
 e.program_title, 
 e.episode_id, 
 e.episode_title, 
 e.schedule, 
 e.period 
FROM episode e
WHERE e.suspend_flg = 0
AND e.schedule > ?1

ORDER BY e.schedule ASC;