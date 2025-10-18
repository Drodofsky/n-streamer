SELECT e.program_id,
 e.program_title, 
 e.episode_id, 
 e.episode_title, 
 e.schedule, 
 e.period, 
 p.genre, 
 p.logo_link, 
 p.synopsis
FROM episode e
JOIN program p
ON e.program_id = p.id
WHERE e.suspend_flg = 0
AND e.schedule > ?1

ORDER BY e.schedule ASC;