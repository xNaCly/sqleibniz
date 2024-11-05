# sqleibniz

A static analysis tool for sql, check syntax errors as well as semantic errors on snippets or full schemata

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

## Configuration

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
        "NoContent",
        "NoStatements", 
        "Unimplemented",

        # ignoring sqlite specific diagnostics:
        # "UnterminatedString", # a not closed string was found
        # "UnknownCharacter", # an unknown character was found
        # "InvalidNumericLiteral", # an invalid numeric literal was found
        # "InvalidBlob", # an invalid blob literal was found (either bad hex data or incorrect syntax)
        # "Syntax", # a structure with incorrect syntax was found
        # "Semicolon", # a semicolon is missing
    ]
```
