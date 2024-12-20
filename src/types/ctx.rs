use std::{collections::HashSet, fmt::Display};
/// https://sqlite.org/datatype3.html#storage_classes_and_datatypes
#[derive(Debug)]
pub enum SqliteStorageClass {
    Null,
    Integer,
    Real,
    Text,
    Blob,
}

trait StrExtension {
    /// returns if s contains any of the elements of v
    fn contains_any(self, v: Vec<&str>) -> bool;
}

impl StrExtension for &str {
    fn contains_any(self, v: Vec<&str>) -> bool {
        for e in v {
            if self.contains(e) {
                return true;
            }
        }
        false
    }
}

impl Display for SqliteStorageClass {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match *self {
            Self::Null => write!(f, "SqliteStorageClass::Null"),
            Self::Integer => write!(f, "SqliteStorageClass::Integer"),
            Self::Real => write!(f, "SqliteStorageClass::Real"),
            Self::Text => write!(f, "SqliteStorageClass::Text"),
            Self::Blob => write!(f, "SqliteStorageClass::Blob"),
        }
    }
}

impl SqliteStorageClass {
    /// https://sqlite.org/datatype3.html#determination_of_column_affinity
    fn from_str(s: &str) -> Self {
        if s.contains_any(vec!["VARCHAR", "CLOB", "TEXT"]) {
            Self::Text
        } else if s.is_empty() || s.contains("BLOB") {
            Self::Blob
        } else if s.contains_any(vec!["REAL", "FLOA", "DOUB"]) {
            Self::Real
        } else if s.contains("INT") {
            Self::Integer
        } else {
            // includes TRUE, FALSE and anything else
            Self::Integer
        }
    }
}

pub struct Table {
    pub name: String,
    pub columns: Vec<SqliteStorageClass>,
}

/// Context holds information necessary for the analysis of sql statements.
pub struct Context {
    pub tables: Vec<Table>,
    pub save_points: HashSet<String>,
    pub databases: HashSet<String>,
}

#[derive(Debug, Clone)]
pub struct HookContext {
    /// [Self::kind] will be the name of the node for most nodes, except nodes that hold different kinds, such as Literal, which can be an Ident, a String, a Number, etc.
    pub kind: String,
    /// [Self::content] holds the textual representation of a nodes contents if it is [crates::parser::nodes::Literal].
    pub content: Option<String>,
    pub children: Vec<HookContext>,
}

impl mlua::IntoLua for HookContext {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        let table = lua.create_table()?;
        table.set("kind", self.kind)?;
        table.set("text", self.content.unwrap_or_else(|| String::new()))?;
        table.set("children", self.children)?;
        lua.pack(table)
    }
}
