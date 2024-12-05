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
