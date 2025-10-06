SELECT * FROM episode 
WHERE suspend_flg = 0
AND schedule > ?1

ORDER BY schedule ASC;