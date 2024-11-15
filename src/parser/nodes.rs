use crate::types::Token;

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
            $(
                pub $field_name: $field_type,
            )*
        }
        impl Node for $node_name {
            fn token(&self) -> &Token {
                &self.t
            }
        }
    };
}

pub trait Node: std::fmt::Debug {
    fn token(&self) -> &Token;
    // TODO: every node should analyse its own contents after the ast was build, to do so the Node
    // trait should enforce a analyse(&self) -> Vec<Error> function.
}
node!(
    Literal,
    "holds all literal types, such as strings, numbers, etc.",
);

node!(Explain,"Explain stmt, see: https://www.sqlite.org/lang_explain.html", child: Option<Box<dyn Node>>);

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
