CALL aoc_load_file_lines('02');
CALL aoc_input_extract_cells('02', '\t', 'INT');

UPDATE "02.output" SET result = (
	SELECT SUM(row.difference)
	FROM (
		SELECT MAX(value::INT) - MIN(value::INT) AS difference
		FROM "02.input.cells"
		GROUP BY row
	) row
) WHERE part = 1;

UPDATE "02.output" SET result = (
	SELECT SUM(a.value / b.value)
	FROM "02.input.cells" a
	INNER JOIN "02.input.cells" b ON a.row = b.row AND a.value != b.value AND a.value % b.value = 0
) WHERE part = 2;

SELECT aoc_results('02');
