use crate::types::{storage::SqliteStorageClass, Keyword, Token};

/// SchemaTableContainer contains either schema_name.table_name or table_name
#[derive(Debug)]
pub enum SchemaTableContainer {
    /// schema_name.table_name
    SchemaAndTable { schema: String, table: String },
    /// table_name
    Table(String),
}

/// Generates a Node from the given input:
///
///```
///node!(
///    Literal,
///    "holds all literal types, such as strings, numbers, etc.",
///);
///#[derive(Debug)]
///#[doc = "holds all literal types, such as strings, numbers, etc."]
///pub struct Literal {
///    #[doc = r" predefined for all structures defined with the node! macro, holds the token of the ast node"]
///    pub t:Token,pub children:Option<Vec<Box<dyn Node>>>,
///}
///impl Node for Literal {
///    fn token(&self) ->  &Token {
///        &self.t
///    }
///    #[cfg(feature = "trace")]
///    fn display(&self,indent:usize){
///        print!("{}- {}"," ".repeat(indent),self.name());
///        println!();
///        if let Some(children) =  &self.children {
///            for child in children {
///                child.display(indent+1)
///            }
///        }
///    }
///    fn name(&self) ->  &str {
///        stringify!(Literal)
///    }
///}
///```
macro_rules! node {
    ($node_name:ident,$documentation:literal,$($field_name:ident:$field_type:ty),*) => {
        #[derive(Debug)]
        #[doc = $documentation]
        pub struct $node_name {
            /// predefined for all structures defined with the node! macro, holds the token of the ast node
            pub t: Token,
            $(
                pub $field_name: $field_type,
            )*
        }
        impl Node for $node_name {
            fn token(&self) -> &Token {
                &self.t
            }

            #[cfg(feature = "trace")]
            fn display(&self, indent: usize) {
                print!("{}- {}({:?})", " ".repeat(indent), self.name(), self.t.ttype);
                $(
                    print!(" [{}={:?}] ", stringify!($field_name), self.$field_name);
                )*
                println!();
            }

            fn name(&self) -> &str {
                stringify!($node_name)
            }
        }
    };
}

pub trait Node: std::fmt::Debug {
    fn token(&self) -> &Token;
    #[cfg(feature = "trace")]
    fn display(&self, indent: usize);
    fn name(&self) -> &str;
    // TODO: every node should analyse its own contents after the ast was build, to do so the Node
    // trait should enforce a analyse(&self, ctx &types::ctx::Context) -> Vec<Error> function.
}

node!(
    Literal,
    "holds all literal types, such as strings, numbers, etc.",
);

node!(
    Expr,
    "Expr expression, see: https://www.sqlite.org/lang_expr.html#varparam",
    literal: Option<Token>,
    bind: Option<BindParameter>,
    schema: Option<String>,
    table: Option<String>,
    column: Option<String>
);

node!(
    BindParameter,
    "Bind parameter: https://www.sqlite.org/lang_expr.html#parameters",
    counter: Option<Box<dyn Node>>,
    name: Option<String>
);

node!(
    Explain,
    "Explain stmt, see: https://www.sqlite.org/lang_explain.html",
    child: Box<dyn Node>
);

node!(Vacuum,"Vacuum stmt, see: https://www.sqlite.org/lang_vacuum.html", schema_name: Option<Token>, filename: Option<Token>);

node!(
    Begin,
    "Begin stmt, see: https://www.sqlite.org/syntax/begin-stmt.html",
);

node!(
    Commit,
    "Commit stmt, see: https://www.sqlite.org/syntax/commit-stmt.html",
);

node!(
    Rollback,
    "Rollback stmt, see: https://www.sqlite.org/syntax/rollback-stmt.html",
    save_point: Option<String>
);

node!(
    Detach,
    "Rollback stmt, see: https://www.sqlite.org/syntax/rollback-stmt.html",
    schema_name: String
);

node!(
    Analyze,
    "Analyze stmt, see: https://www.sqlite.org/lang_analyze.html",
    target: Option<SchemaTableContainer>
);

node!(
    Drop,
    "Drop stmt, see: https://www.sqlite.org/lang_dropindex.html, https://www.sqlite.org/lang_droptable.html, https://www.sqlite.org/lang_droptrigger.html and https://www.sqlite.org/lang_dropview.html",
    if_exists: bool,
    ttype: Keyword,
    argument: String
);

node!(
    Savepoint,
    "Savepoint stmt, see: https://www.sqlite.org/lang_savepoint.html",
    savepoint_name: String
);

node!(
    Release,
    "Release stmt, see: https://www.sqlite.org/lang_savepoint.html",
    savepoint_name: String
);

node!(
    Attach,
    "Attach stmt, see: https://www.sqlite.org/lang_attach.html",
    schema_name: String,
    expr: Expr
);

node!(
    Reindex,
    "Reindex stmt, see: https://www.sqlite.org/lang_reindex.html",
    target: Option<SchemaTableContainer>
);

node!(
    Alter,
    "Alter stmt, see: https://www.sqlite.org/lang_altertable.html
SQLite supports a limited subset of ALTER TABLE. The ALTER TABLE command in SQLite allows these alterations of an existing table: it can be renamed; a column can be renamed; a column can be added to it; or a column can be dropped from it.",
    target: SchemaTableContainer,
    rename_to: Option<String>,
    rename_column_target: Option<String>,
    new_column_name: Option<String>,
    add_column: Option<ColumnDef>, // TODO: think about a data structure for this
    drop_column: Option<String>
);

node!(
    ColumnDef,
    "Column definition, see: https://www.sqlite.org/syntax/column-def.html",
    name: String,
    // equivalent to type_name: https://www.sqlite.org/syntax/type-name.html
    type_name: Option<SqliteStorageClass>
    // TODO: should i include this in analyis?
    // constraint: Option<()>
);
