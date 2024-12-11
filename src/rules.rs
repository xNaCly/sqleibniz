use mlua::{FromLua, Function, IntoLua, Table, UserData};

use crate::types::HookContext;

#[derive(Debug, PartialEq, Clone)]
/// Rule is attached to each error and can be supplied to sqleibniz via the Config structure serialized in ./leibniz.toml
#[derive(clap::ValueEnum)]
pub enum Rule {
    /// Source file is empty
    NoContent,
    /// Source file is not empty but holds no statements
    NoStatements,
    /// Source file contains constructs sqleibniz does not yet understand
    Unimplemented,
    /// Source file contains an unknown keyword
    UnknownKeyword,
    /// Source file contains invalid sqleibniz instruction
    BadSqleibnizInstruction,

    /// Source file contains an unterminated string
    UnterminatedString,
    /// The source file contains an unknown character
    UnknownCharacter,
    /// The source file contains an invalid numeric literal, either overflow or incorrect syntax
    InvalidNumericLiteral,
    /// The source file contains an invalid blob literal, either bad hex data (a-f,A-F,0-9) or
    /// incorrect syntax
    InvalidBlob,
    /// The source file contains a structure with incorrect syntax
    Syntax,
    /// The source file is missing a semicolon
    Semicolon,
}

impl FromLua for Rule {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        let value: String = lua.unpack(value)?;

        Ok(match value.as_str() {
            "NoContent" => Self::NoContent,
            "NoStatements" => Self::NoStatements,
            "Unimplemented" => Self::Unimplemented,
            "UnterminatedString" => Self::UnterminatedString,
            "UnknownCharacter" => Self::UnknownCharacter,
            "InvalidNumericLiteral" => Self::InvalidNumericLiteral,
            "InvalidBlob" => Self::InvalidBlob,
            "Syntax" => Self::Syntax,
            "Semicolon" => Self::Semicolon,
            "BadSqleibnizInstruction" => Self::BadSqleibnizInstruction,
            "UnknownKeyword" => Self::UnknownKeyword,
            _ => {
                return Err(mlua::Error::FromLuaConversionError {
                    from: "string",
                    to: "sqleibniz::rules::Rule".into(),
                    message: Some("Unknown rule name".into()),
                })
            }
        })
    }
}

#[derive(Debug)]
/// Configuration is expected to be at ./leibniz.lua - its existence is not required for the program invocation
pub struct Config {
    /// holds the rules that the user wants to not see errors for.
    pub disabled_rules: Vec<Rule>,
    /// holds the hooks the user wants to execute
    pub hooks: Option<Vec<Hook>>,
}

impl FromLua for Config {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        let table: Table = lua.unpack(value)?;
        let disabled_rules: Vec<Rule> = table.get("disabled_rules").unwrap_or_else(|_| vec![]);
        let hooks: Option<Vec<Hook>> = table.get("hooks").ok();
        Ok(Self {
            disabled_rules,
            hooks,
        })
    }
}

#[derive(Debug)]
/// sqleibniz allows for writing custom rules with lua
pub struct Hook {
    pub name: String,
    pub node: Option<String>,
    /// hook can be executed via [Function::call]`(args)`, where args is [HookContext]
    pub hook: Option<Function>,
}

impl Hook {
    pub fn exec(&self, arg: HookContext) -> mlua::Result<()> {
        if let Some(hook) = &self.hook {
            hook.call(arg)?
        }
        Ok(())
    }
}

impl IntoLua for Hook {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        lua.pack(lua.create_table()?)
    }
}

impl FromLua for Hook {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        let table: Table = lua.unpack(value)?;
        let name = table.get("name")?;
        let node = table.get("node").ok();
        let hook: Option<Function> = table.get("hook").ok();
        // INFO: lua function call example
        //
        // if let Some(h) = &hook {
        //     let () = dbg!(h.call(HookContext {
        //         kind: "ident".into(),
        //         text: Some("UPPERCASE".into()),
        //         children: vec![],
        //     }))?;
        // }
        Ok(Self { name, node, hook })
    }
}

impl UserData for Config {}
impl UserData for Rule {}

impl Rule {
    pub fn name(&self) -> &str {
        match self {
            Self::NoContent => "NoContent",
            Self::NoStatements => "NoStatements",
            Self::Unimplemented => "Unimplemented",
            Self::UnterminatedString => "UnterminatedString",
            Self::UnknownCharacter => "UnknownCharacter",
            Self::InvalidNumericLiteral => "InvalidNumericLiteral",
            Self::InvalidBlob => "InvalidBlob",
            Self::Syntax => "Syntax",
            Self::Semicolon => "Semicolon",
            Self::BadSqleibnizInstruction => "BadSqleibnizInstruction",
            Self::UnknownKeyword => "UnknownKeyword",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Self::NoContent => "Source file is empty",
            Self::NoStatements => "Source file is not empty but holds no statements",
            Self::Unimplemented => {
                "Source file contains constructs sqleibniz does not yet understand"
            }
            Self::UnterminatedString => "Source file contains an unterminated string",
            Self::UnknownCharacter => "The source file contains an unknown character",
            Self::InvalidNumericLiteral => "The source file contains an invalid numeric literal",
            Self::InvalidBlob => "The source file contains an invalid blob literal",
            Self::Syntax => "The source file contains a structure with incorrect syntax",
            Self::Semicolon => "The source file is missing a semicolon",
            Self::BadSqleibnizInstruction => {
                "The source file contains an invalid sqleibniz instruction"
            }
            Self::UnknownKeyword => "Source file contains an unknown keyword",
        }
    }
}
