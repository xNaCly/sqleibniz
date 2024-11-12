# sqleibniz

A static analysis tool for sql, check syntax errors as well as semantic errors on snippets or full schemata

> [!WARNING]  
> Sqleibniz is in early stages of development, please keep this in mind before
> creating issues - contributions are always welcome 💗

## Features

Sqleibniz is a command line tool to analyse sql statements by checking for their static and
dynamic correctness. See below for a list of currently implemented features.

### Supported features

- [ ] static analysis (syntax and semantic analysis)
  - [x] syntax analysis - sqleibniz aims to implement the syntax [sqlite understands](https://www.sqlite.org/lang.html)
  - [ ] warn for sqlites [quirks](https://www.sqlite.org/quirks.html)
  - [ ] do the used tables exist / were they created beforehand
  - [ ] do the used columns exist / were they created beforehand
  - [ ] do the used functions exist / were they created beforehand
  - [ ] are all used types compatible
- [ ] dynamic analysis (runtime analysis via embedded sqlite)
  - [ ] assertions via `@sqleibniz::assert`
  - [ ] were all tables and their columns created correctly (with correct storage classes)
  - [ ] were all stmts executed successfully
- [ ] pretty errors
  - [x] faulty code display with line numbers
  - [x] link to sqlite documentation for each diagnostic
  - [x] ability to omit specific errors depending on their group (Rule)
  - [x] highlighting the error in the faulty code snippet
  - [x] explanation why the specific error was ommitted based on its Rule
  - [ ] possible fix suggestions

### Supported sql statements

| done | `sqlite`-syntax name        | sql example                     | non-standard sql |
| ---- | --------------------------- | ------------------------------- | ---------------- |
| ✅   | `explain-stmt`              | `EXPLAIN QUERY PLAN;`           |                  |
|      | `alter-table-stmt`          |                                 |                  |
|      | `analyze-stmt`              |                                 |                  |
|      | `attach-stmt`               |                                 |                  |
| ✅   | `begin-stmt`                | `BEGIN DEFERRED TRANSACTION;`   |                  |
| ✅   | `commit-stmt`               | `END TRANSACTION;`              |                  |
|      | `create-index-stmt`         |                                 |                  |
|      | `create-table-stmt`         |                                 |                  |
|      | `create-trigger-stmt`       |                                 |                  |
|      | `create-view-stmt`          |                                 |                  |
|      | `create-virtual-table-stmt` |                                 |                  |
|      | `delete-stmt`               |                                 |                  |
|      | `delete-stmt-limited`       |                                 |                  |
|      | `detach-stmt`               |                                 |                  |
|      | `drop-index-stmt`           |                                 |                  |
|      | `drop-table-stmt`           |                                 |                  |
|      | `drop-view-stmt`            |                                 |                  |
|      | `insert-stmt`               |                                 |                  |
|      | `pragma-stmt`               |                                 | sqlite specific  |
|      | `reindex-stmt`              |                                 |                  |
|      | `release-stmt`              |                                 |                  |
| ✅   | `rollback-stmt`             | `ROLLBACK TO latest_savepoint;` |                  |
|      | `savepoint-stmt`            |                                 |                  |
|      | `select-stmt`               |                                 |                  |
|      | `update-stmt`               |                                 |                  |
|      | `update-stmt-limited`       |                                 |                  |
| ✅   | `vacuum-stmt`               | `VACUUM INTO 'repacked.db'`     |                  |

## Installation

### cargo

```
cargo install --git https://github.com/xnacly/sqleibniz
```

#### from source

```shell
git clone https://github.com/xnacly/sqleibniz
cargo install --path .
```

### via `make`

> this builds the project with cargo and moves the resulting binary to
> `/usr/bin/`.

```shell
git clone https://github.com/xnacly/sqleibniz
make
```

Uninstall via:

```shell
make uninstall
```

## Usage

```shell
sqleibniz <file>
sqleibniz <file1> <file2>
```

### Configuration

Sqleibniz can be configured via a `leibniz.toml` file, this file has to be
accessible to sqleibniz by existing at the path sqleibniz is invoked at.
Consult [src/rules.rs](./src/rules.rs) for configuration documentation and
[leibniz.toml](./leibniz.toml) for said example:

```toml
# this is an example file, consult: https://toml.io/en/ and src/rules.rs for
# documentation
[disabled]
    rules = [
        # by default, sqleibniz specific errors are disabled:
        "NoContent", # source file is empty
        "NoStatements", # source file contains no statements
        "Unimplemented", # construct is not implemented yet
        "BadSqleibnizInstruction", # source file contains a bad sqleibniz instruction

        # ignoring sqlite specific diagnostics:
        # "UnterminatedString", # a not closed string was found
        # "UnknownCharacter", # an unknown character was found
        # "InvalidNumericLiteral", # an invalid numeric literal was found
        # "InvalidBlob", # an invalid blob literal was found (either bad hex data or incorrect syntax)
        # "Syntax", # a structure with incorrect syntax was found
        # "Semicolon", # a semicolon is missing
    ]
```

### sqleibniz instructions

A sqleibniz instrution is prefixed with `@sqleibniz::` and written inside of a
sql single line comment.

#### `expect`

In a similar fashion to ignoring diagnostics via the configuration in
`leibniz.toml`, sqleibniz allows the user to expect diagnostics in the source
file and omit them on a statement by statement basis. To do so, a comment
containing a sqleibniz instruction has to be issued:

```sql
-- will not cause a diagnostic
-- @sqleibniz::expect <explanation for instruction usage here>
-- incorrect, because EXPLAIN wants a sql stmt
EXPLAIN 25;

-- will not cause a diagnostic
-- @sqleibniz::expect <explanation for instruction usage here>
-- incorrect, because 'unknown_table' does not exist
SELECT * FROM unknown_table;

-- will cause a diagnostic
-- incorrect, because EXPLAIN wants a sql stmt, not a literal
EXPLAIN QUERY PLAN 25;
```

Passing the above file to `sqleibniz`:

```text
warn: Ignoring the following diagnostics, according to 'leibniz.toml':
 -> NoContent
 -> NoStatements
 -> Unimplemented
 -> BadSqleibnizInstruction
======================== ./tests/sqleibniz.sql =========================
error[Syntax]: Unexpected Literal
 -> /home/teo/programming/sqleibniz/tests/sqleibniz.sql:12:20
 10 | -- will cause a diagnostic
 11 | -- incorrect, because EXPLAIN wants a sql stmt, not a literal
 12 | EXPLAIN QUERY PLAN 25;
    |                    ^^ error occurs here.
    |
    ~ note: Literal Number(25.0) disallowed at this point.
  * Syntax: The source file contains a structure with incorrect syntax

 docs: https://www.sqlite.org/syntax/sql-stmt.html
=============================== Summary ================================
[-] ./tests/sqleibniz.sql:
    1 Error(s) detected
    0 Error(s) ignored

=> 0/1 Files verified successfully, 1 verification failed.
```

`@sqleibniz::expect` is implemented by inserting a token with the type
`Type::InstructionExpect`. The parser encounters this token and consumes all
token until a token with the type `Type::Semicolon` is found. Thus sqleibniz is
skipping the analysis of the statement directly after the sqleibniz
instruction. A statement is terminated via `;`. `@sqleibniz::expect` therefore
supports ignoring diagnostics for statements spanning either a single line or
multiple lines.
