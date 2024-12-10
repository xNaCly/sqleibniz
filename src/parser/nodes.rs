use crate::types::{Keyword, Token};

/// Generates a Node from the given input:
///
///     node!(Literal,);
///     // generates:
///     #[derive(Debug)]
///     pub struct Literal {
///         pub t: Token,
///     }
///     impl Node for Literal {
///         fn token(&self) -> &Token {
///             &self.t
///         }
///     }
macro_rules! node {
    ($node_name:ident,$documentation:literal,$($field_name:ident:$field_type:ty),*) => {
        #[derive(Debug)]
        #[doc = $documentation]
        pub struct $node_name {
            /// predefined for all structures defined with the node! macro, holds the token of the ast node
            pub t: Token,
            pub children: Option<Vec<Box<dyn Node>>>,
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
                print!("{}- {}", " ".repeat(indent), self.name());
                $(
                    print!(" [{}={:?}] ", stringify!($field_name), self.$field_name);
                )*
                println!();
                if let Some(children) = &self.children {
                    for child in children {
                        child.display(indent+1)
                    }
                }
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
    // trait should enforce a analyse(&self) -> Vec<Error> function.
}

node!(
    Literal,
    "holds all literal types, such as strings, numbers, etc.",
);

node!(
    Explain,
    "Explain stmt, see: https://www.sqlite.org/lang_explain.html",
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
    "Analyze stmt, see: https://www.sqlite.org/syntax/lang_analyze.html",
    schema_index_or_table_name: Option<String>,
    schema_with_table_or_index_name: Option<String>
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
    schema_name: String
);

node!(
    Reindex,
    "Reindex stmt, see: https://www.sqlite.org/lang_reindex.html",
    collation_or_schema: Option<String>
);
