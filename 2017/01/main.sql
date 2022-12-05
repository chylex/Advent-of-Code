CALL aoc_load_file_lines('01');

CREATE TABLE "01.characters" AS
	SELECT r.row - 1 AS index, r.character
	FROM "01.input"
	CROSS JOIN REGEXP_SPLIT_TO_TABLE(input, '') WITH ORDINALITY AS r(character, row);

CREATE UNIQUE INDEX "01.characters.index" ON "01.characters" (index);

UPDATE "01.output"
SET result = (
	SELECT SUM(a.character::INT) AS result
	FROM "01.characters" a
	INNER JOIN "01.characters" b ON a.index = (b.index + 1) % (SELECT MAX(index) + 1 FROM "01.characters")
	WHERE a.character = b.character
)
WHERE part = 1;

UPDATE "01.output"
SET result = (
	SELECT SUM(a.character::INT) AS result
	FROM "01.characters" a
	INNER JOIN "01.characters" b ON a.index = (b.index + (SELECT (MAX(index) + 1) / 2 FROM "01.characters")) % (SELECT MAX(index) + 1 FROM "01.characters")
	WHERE a.character = b.character
)
WHERE part = 2;

SELECT aoc_results('01')
