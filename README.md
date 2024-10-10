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




```shell
$ ./target/release/sqleibniz 
error: no source file(s) provided, exiting
$ ./target/release/sqleibniz tests/*
warn: Ignoring the following diagnostics, according to 'leibniz.toml':
 -> NoContent
 -> NoStatements
 -> Unimplemented
=========================== tests/string.sql ===========================
error[UnterminatedString]: Unterminated String in 'tests/string.sql'
 => tests/string.sql
 01 | /**/    'string1
    |         ^^^^^^^^ error occurs here.
 02 | -- this should work1
 03 | 'string2'
    |
    = note: Consider adding a "'" at the end of this string

error[UnterminatedString]: Unterminated String in 'tests/string.sql'
 => tests/string.sql
 02 | -- this should work1
 03 | 'string2'
 04 | 'string3
    | ^^^^^^^^ error occurs here.
 05 | -- this should work2
 06 | 'string4'
    |
    = note: Consider adding a "'" at the end of this string
=============================== Summary ================================
[+] tests/comment.sql:
    0 Error(s) detected
    1 Error(s) ignored
[+] tests/empty.sql:
    0 Error(s) detected
    1 Error(s) ignored
[+] tests/select.sql:
    0 Error(s) detected
    1 Error(s) ignored
[-] tests/string.sql:
    2 Error(s) detected
    0 Error(s) ignored

=> 3/4 Files verified successfully, 1 verification failed.
```

Screenshot:
![image](https://github.com/user-attachments/assets/b4e72546-be7e-4a3a-9d8c-fa195de37e65)


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
rules = [ "NoContent", "NoStatements", "Unimplemented" ]
```
