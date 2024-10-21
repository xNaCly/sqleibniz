/*

stmt.sql displays the current progress of sqleibniz by highlighting all
currently available statements.

*/


-- https://www.sqlite.org/lang_explain.html
-- this is NULL, because the parser wants a statement after this and the parser doesnt yet support one
EXPLAIN NULL;
EXPLAIN QUERY PLAN NULL;

-- https://www.sqlite.org/lang_vacuum.html
VACUUM;
VACUUM schema_name;
VACUUM INTO 'filename';
VACUUM schema_name INTO 'filename';
