CALL aoc_text_file('02');

UPDATE "02.output" SET result = (
	SELECT SUM(row.difference)
	FROM (
		SELECT MAX(cell::INTEGER) - MIN(cell::INTEGER) AS difference
		FROM "02.input"
		CROSS JOIN REGEXP_SPLIT_TO_TABLE(input, '\t') cell
		GROUP BY input
	) row
) WHERE part = 1;

SELECT aoc_results('02');
