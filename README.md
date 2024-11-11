# sqleibniz

A static analysis tool for sql, check syntax errors as well as semantic errors on snippets or full schemata


> [!WARNING]  
> Sqleibniz is in early stages of development, please keep this in mind before
> creating issues - contributions are always welcome 💗

## Features

- static sql analysis:
  - is the syntax correct? - sqleibniz aims to implement the syntax [sqlite understands](https://www.sqlite.org/lang.html)
  - do the used tables and columns exist?
  - do the used functions exist?
  - are the types of all operations compatible and produce the expected result?
- runtime sql analysis
  - executing input via embedded memory sqlite
  - automatically examining the resulting database and generating a report containing:
    - created tables, their columns with types and the amount of rows in each table
    - a list of statements that failed and the returned error
- very pretty errors :^)
  - source location
  - sql syntax highlighting
  - hints for possible fixes and
  - link to the corresponding sqlite page

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

