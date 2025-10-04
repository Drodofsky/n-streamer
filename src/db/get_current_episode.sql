SELECT * FROM episode
WHERE schedule > ?1
AND suspend_flg = 0
ORDER BY schedule ASC
LIMIT 1;