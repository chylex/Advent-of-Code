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
	RETURN QUERY EXECUTE FORMAT('SELECT CONCAT(''Part '', part, '' : '', result) FROM %I WHERE result IS NOT NULL ORDER BY part', day || '.output');
END
$$ LANGUAGE plpgsql;

CREATE OR REPLACE PROCEDURE aoc_text_file(day TEXT) AS
$$
BEGIN
	CALL aoc_setup_tables(day, 'input TEXT');
	EXECUTE FORMAT('COPY %I FROM ''/aoc/%s/input.txt''', day || '.input', day);
END
$$ LANGUAGE plpgsql;
