use std::collections::HashSet;

use super::storage::SqliteStorageClass;

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
        table.set("text", self.content.unwrap_or_default())?;
        table.set("children", self.children)?;
        lua.pack(table)
    }
}
