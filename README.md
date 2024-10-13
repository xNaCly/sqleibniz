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

<!-- TODO: -->

```shell

```

## Usage

Screenshot:
![image](https://github.com/user-attachments/assets/bb19dba6-1872-4ffa-a04e-e616cf492b40)



## Configuration

Sqleibniz can be configured via a `leibniz.toml` file, this file has to be
accessible to sqleibniz by existing at the path sqleibniz is invoked at.
Consult [src/rules.rs](./src/rules.rs) for configuration documentation and
[leibniz.toml](./leibniz.toml) for said example:

```toml
# this is an example file, consult: https://toml.io/en/ and src/rules.rs for
# documentation
[disabled] 
# by default, sqleibniz specific errors are disabled:
rules = [ 
    "NoContent",
    "NoStatements", 
    "Unimplemented" 
]
```
