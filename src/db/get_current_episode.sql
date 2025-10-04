SELECT * FROM episode
WHERE schedule > ?1
ORDER BY schedule ASC
LIMIT 1;