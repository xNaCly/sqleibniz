-- will not cause a diagnostic
-- @sqleibniz::expect <explanation for instruction usage here>
-- incorrect, because EXPLAIN wants a sql stmt
EXPLAIN 25; 

-- will not cause a diagnostic
-- @sqleibniz::expect incorrect, because deleted_table does not exist
SELECT * FROM deleted_table;

-- will not cause a diagnostic
-- @sqleibniz::expect incorrect, because EXPLAIN wants a sql stmt, not a literal
EXPLAIN QUERY PLAN 25;
