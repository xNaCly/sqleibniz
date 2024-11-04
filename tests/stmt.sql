/* stmt.sql displays the current progress of sqleibniz by highlighting all currently available statements. */

-- https://www.sqlite.org/lang_explain.html
EXPLAIN SELECT;
EXPLAIN QUERY PLAN VACUUM;

-- https://www.sqlite.org/lang_vacuum.html
VACUUM;
VACUUM schema_name;
VACUUM INTO 'filename';
VACUUM schema_name INTO 'filename';
