--- @diagnostic disable: lowercase-global

-- this is an example configuration, consult: https://www.lua.org/manual/5.4/
-- or https://learnxinyminutes.com/docs/lua/ for syntax help and
-- src/rules.rs::Config for all available options
leibniz = {
    disabled_rules = {
        -- ignore sqleibniz specific diagnostics:
        "NoContent",               -- source file is empty
        "NoStatements",            -- source file contains no statements
        "Unimplemented",           -- construct is not implemented yet
        "BadSqleibnizInstruction", -- source file contains a bad sqleibniz instruction

        -- ignore sqlite specific diagnostics:

        -- "UnknownKeyword", -- an unknown keyword was encountered
        -- "UnterminatedString", -- a not closed string was found
        -- "UnknownCharacter", -- an unknown character was found
        -- "InvalidNumericLiteral", -- an invalid numeric literal was found
        -- "InvalidBlob", -- an invalid blob literal was found (either bad hex data or incorrect syntax)
        -- "Syntax", -- a structure with incorrect syntax was found
        -- "Semicolon", -- a semicolon is missing
    },
    -- sqleibniz allows for writing custom rules with lua
    -- https://github.com/mlua-rs/mlua/issues/426
    hooks = {
        {
            -- summarises the hooks content
            name = "idents should be lowercase",
            -- instructs sqleibniz which node to execute the `hook` for
            node = "literal",
            -- sqleibniz calls the hook function once it encounters a node name
            -- matching the hook.node content
            --
            -- The `node` argument holds the following fields:
            --
            --```
            --node: {
            -- type: string,
            -- text: string,
            -- children: node[],
            --}
            --```
            --
            hook = function(node)
                if node.type == "ident" then
                    if string.match(node.text, "%u") then
                        -- returing an error passes the diagnostic to sqleibniz,
                        -- thus a pretty message with the name of the hook, the
                        -- node it occurs and the message passed to error() is
                        -- generated
                        error("All idents should be lowercase")
                    end
                end
            end
        }
    }
}
