/* stmt.sql displays the current progress of sqleibniz by highlighting all currently available statements. */

-- https://www.sqlite.org/lang_explain.html
EXPLAIN VACUUM;
EXPLAIN QUERY PLAN VACUUM;

-- https://www.sqlite.org/lang_vacuum.html
VACUUM;
VACUUM schema_name;
VACUUM INTO 'filename';
VACUUM schema_name INTO 'filename';

-- https://www.sqlite.org/lang_transaction.html
-- https://www.sqlite.org/syntax/begin-stmt.html
BEGIN;
BEGIN TRANSACTION;
BEGIN DEFERRED;
BEGIN IMMEDIATE;
BEGIN EXCLUSIVE;
BEGIN DEFERRED TRANSACTION;
BEGIN IMMEDIATE TRANSACTION;
BEGIN EXCLUSIVE TRANSACTION;
