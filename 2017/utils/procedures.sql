CREATE OR REPLACE PROCEDURE aoc_drop_tables(day TEXT) AS
$$
DECLARE
	row RECORD;
BEGIN
	FOR row IN
		SELECT table_name
		FROM information_schema.tables
		WHERE table_name LIKE (day || '.%') AND table_schema = CURRENT_SCHEMA()
	LOOP
		EXECUTE FORMAT('DROP TABLE %I CASCADE', row.table_name);
	END LOOP;
END
$$ LANGUAGE plpgsql;

CREATE OR REPLACE PROCEDURE aoc_setup_tables(day TEXT, input_columns TEXT) AS
$$
BEGIN
	CALL aoc_drop_tables(day);
	EXECUTE FORMAT('CREATE TABLE %I (%s)', day || '.input', input_columns);
	EXECUTE FORMAT('CREATE TABLE %I (part INT, result TEXT NULL)', day || '.output');
	EXECUTE FORMAT('INSERT INTO %I (part) VALUES (1), (2)', day || '.output');
END
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION aoc_results(day TEXT)
	RETURNS TABLE (
		RESULT TEXT
	)
	STABLE
	ROWS 2
AS
$$
BEGIN
	RETURN QUERY EXECUTE FORMAT('SELECT result FROM %I ORDER BY part', day || '.output');
END
$$ LANGUAGE plpgsql;

CREATE OR REPLACE PROCEDURE aoc_load_file_lines(day TEXT) AS
$$
BEGIN
	CALL aoc_setup_tables(day, 'input TEXT');
	EXECUTE FORMAT('COPY %I FROM ''/aoc/%s/input.txt'' WITH DELIMITER E''\1''', day || '.input', day);
END
$$ LANGUAGE plpgsql;

CREATE OR REPLACE PROCEDURE aoc_input_extract_cells(day TEXT, delimiter TEXT, value_type TEXT) AS
$$
BEGIN
	EXECUTE FORMAT('CREATE TABLE %I AS
		SELECT input.row, cell.col, cell.value::%s
		FROM (SELECT ROW_NUMBER() OVER () AS row, input AS line FROM %I) input
		CROSS JOIN REGEXP_SPLIT_TO_TABLE(input.line, %L) WITH ORDINALITY AS cell(value, col)', day || '.input.cells', value_type, day || '.input', delimiter);
END
$$ LANGUAGE plpgsql;
