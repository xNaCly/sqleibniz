-- both EXPLAIN paths

-- this is NULL, because the parser wants a statment after this and the parser doesnt yet support one
EXPLAIN NULL;
EXPLAIN QUERY PLAN NULL;

VACUUM;
VACUUM schema_name;
VACUUM INTO 'filename';
VACUUM schema_name INTO 'filename';
