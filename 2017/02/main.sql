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

CREATE TABLE "02.cells" AS
	SELECT input.row, cell.col, cell.value::INTEGER
	FROM (SELECT ROW_NUMBER() OVER () AS row, input AS line FROM "02.input") input
	CROSS JOIN REGEXP_SPLIT_TO_TABLE(input.line, '\t') WITH ORDINALITY AS cell(value, col);

UPDATE "02.output" SET result = (
	SELECT SUM(a.value / b.value)
	FROM "02.cells" a
	INNER JOIN "02.cells" b ON a.row = b.row AND a.value != b.value AND a.value % b.value = 0
) WHERE part = 2;

SELECT aoc_results('02');
