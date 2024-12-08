# sqleibniz

LSP and analysis cli for sql. Check for valid syntax,
semantics and perform dynamic analysis.

> [!WARNING]  
> Sqleibniz is in early stages of development, please keep this in mind before
> creating issues. Contributions are always welcome 💗

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
  - [x] suggestions for unknown and possible misspelled keywords
- [ ] language server protocol
  - [ ] diagnostics for full sqleibniz analysis
  - [ ] snippets
  - [ ] intelligent completions

### Supported sql statements

| done | `sqlite`-syntax name        | sql example                          | non-standard sql |
| ---- | --------------------------- | ------------------------------------ | ---------------- |
| ✅   | `explain-stmt`              | `EXPLAIN QUERY PLAN;`                |                  |
|      | `alter-table-stmt`          |                                      |                  |
| ✅   | `analyze-stmt`              | `ANALYZE my_table;`                  |                  |
|      | `attach-stmt`               |                                      |                  |
| ✅   | `begin-stmt`                | `BEGIN DEFERRED TRANSACTION;`        |                  |
| ✅   | `commit-stmt`               | `END TRANSACTION;`                   |                  |
|      | `create-index-stmt`         |                                      |                  |
|      | `create-table-stmt`         |                                      |                  |
|      | `create-trigger-stmt`       |                                      |                  |
|      | `create-view-stmt`          |                                      |                  |
|      | `create-virtual-table-stmt` |                                      |                  |
|      | `delete-stmt`               |                                      |                  |
|      | `delete-stmt-limited`       |                                      |                  |
| ✅   | `detach-stmt`               | `DETACH DATABASE my_database`        |                  |
| ✅   | `drop-index-stmt`           | `DROP INDEX my_index;`               |                  |
| ✅   | `drop-table-stmt`           | `DROP TABLE my_table;`               |                  |
| ✅   | `drop-trigger-stmt`         | `DROP TRIGGER my_trigger;`           |                  |
| ✅   | `drop-view-stmt`            | `DROP VIEW my_view;`                 |                  |
|      | `insert-stmt`               |                                      |                  |
|      | `pragma-stmt`               |                                      | sqlite specific  |
| ✅   | `reindex-stmt`              | `REINDEX my_schema.my_table`         |                  |
| ✅   | `release-stmt`              | `RELEASE SAVEPOINT latest_savepoint` |                  |
| ✅   | `rollback-stmt`             | `ROLLBACK TO latest_savepoint;`      |                  |
| ✅   | `savepoint-stmt`            | `SAVEPOINT latest_savepoint`         |                  |
|      | `select-stmt`               |                                      |                  |
|      | `update-stmt`               |                                      |                  |
|      | `update-stmt-limited`       |                                      |                  |
| ✅   | `vacuum-stmt`               | `VACUUM INTO 'repacked.db'`          |                  |

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

<!--## Language Server Protocol (lsp)

> [!WARNING]
> This feature is not yet implemented.

### Setup in Neovim

> requires systemwide installation beforehand
-->

## Command line interface usage

```text
LSP and analysis cli for sql. Check for valid syntax, semantics and perform dynamic analysis

Usage: sqleibniz [OPTIONS] [PATHS]...

Arguments:
  [PATHS]...
          files to analyse

Options:
  -i, --ignore-config
          instruct sqleibniz to ignore the configuration, if found

  -c, --config <CONFIG>
          path to the configuration

          [default: leibniz.toml]

  -s, --silent
          disable stdout/stderr output

  -D <DISABLE>
          disable diagnostics by their rules, all are enabled by default - this may change in the future

          Possible values:
          - no-content:                Source file is empty
          - no-statements:             Source file is not empty but holds no statements
          - unimplemented:             Source file contains constructs sqleibniz does not yet understand
          - unknown-keyword:           Source file contains an unknown keyword
          - bad-sqleibniz-instruction: Source file contains invalid sqleibniz instruction
          - unterminated-string:       Source file contains an unterminated string
          - unknown-character:         The source file contains an unknown character
          - invalid-numeric-literal:   The source file contains an invalid numeric literal, either overflow or incorrect syntax
          - invalid-blob:              The source file contains an invalid blob literal, either bad hex data (a-f,A-F,0-9) or incorrect syntax
          - syntax:                    The source file contains a structure with incorrect syntax
          - semicolon:                 The source file is missing a semicolon

  -h, --help
          Print help (see a summary with '-h'
```

### Configuration

Sqleibniz can be configured via a `leibniz.lua` file, this file has to be
accessible to sqleibniz by existing at the path sqleibniz is invoked at.
Consult [src/rules.rs](./src/rules.rs) for configuration documentation and
[leibniz.lua](./leibniz.lua) for said example:

```lua
-- this is an example configuration, consult: https://www.lua.org/manual/5.4/
-- or https://learnxinyminutes.com/docs/lua/ for syntax help and
-- src/rules.rs::Config for all available options
leibniz.disabled_rules = {
    -- by default, sqleibniz specific errors are disabled:
    "NoContent",               -- source file is empty
    "NoStatements",            -- source file contains no statements
    "Unimplemented",           -- construct is not implemented yet
    "BadSqleibnizInstruction", -- source file contains a bad sqleibniz instruction

    -- ignoring sqlite specific diagnostics:

    -- "UnknownKeyword", -- an unknown keyword was encountered
    -- "UnterminatedString", -- a not closed string was found
    -- "UnknownCharacter", -- an unknown character was found
    -- "InvalidNumericLiteral", -- an invalid numeric literal was found
    -- "InvalidBlob", -- an invalid blob literal was found (either bad hex data or incorrect syntax)
    -- "Syntax", -- a structure with incorrect syntax was found
    -- "Semicolon", -- a semicolon is missing
}
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

## Contribution

Contributions are always welcome <3, but remember to test all features you contribute.

### Local Dev env

```shell
git clone git@github.com:xNaCly/sqleibniz.git
cargo run example/*
```

### Debugging the parser

Run sqleibniz via cargo with `--features trace` to enable the log of each
`Parser.<stmt_type>_stmt` function as well as the resulting ast nodes. This
allows for a deeper insight for deadlocks etc.

```sql
EXPLAIN VACUUM;
EXPLAIN QUERY PLAN VACUUM my_big_schema INTO 'repacked.db';
```

For instance, parsing the above SQL results in the generation and print of a
parser callstack and the resulting AST:

```text
sqleibniz master M :: cargo run --features trace -- test.sql
   Compiling sqleibniz v0.1.0 (/home/magr6/programming/sqleibniz)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 2.27s
     Running `target/debug/sqleibniz test.sql`
warn: Ignoring the following diagnostics, as specified:
 -> NoContent
 -> NoStatements
 -> Unimplemented
 -> BadSqleibnizInstruction
============================== CALL STACK ==============================
 ↳ Parser::parse(Some(Keyword(EXPLAIN)))
  ↳ Parser::sql_stmt_list(Some(Keyword(EXPLAIN)))
   ↳ Parser::sql_stmt_prefix(Some(Keyword(EXPLAIN)))
    ↳ Parser::sql_stmt(Some(Keyword(VACUUM)))
     ↳ Parser::vacuum_stmt(Some(Keyword(VACUUM)))
    ↳ Parser::sql_stmt_prefix(Some(Keyword(EXPLAIN)))
     ↳ Parser::sql_stmt(Some(Keyword(VACUUM)))
      ↳ Parser::vacuum_stmt(Some(Keyword(VACUUM)))
================================= AST ==================================
- Explain
 - Vacuum [schema_name=None]  [filename=None]
- Explain
 - Vacuum [schema_name=Some(Token { ttype: Ident("my_big_schema"), start: 26, end: 39, line: 1 })]  [filename=Some(Token { ttype: String("repacked.db"), start: 45, end: 58, line: 1 })]
=============================== Summary ================================
[+] test.sql:
    0 Error(s) detected
    0 Error(s) ignored

=> 1/1 Files verified successfully, 0 verification failed
```
