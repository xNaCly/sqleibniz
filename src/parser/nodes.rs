use crate::types::Token;

/// Generates a Node from the given input:
///
///     node!(Literal, t: Token);
///     // generates:
///     #[derive(Debug)]
///     pub struct Literal {
///         pub t: Token,
///     }
///     impl Node for Literal {}
macro_rules! node {
    ($node_name:ident,$($field_name:ident:$field_type:ty),*) => {
#[derive(Debug)]
pub struct $node_name {
    $(
        pub $field_name: $field_type,
    )*
}
impl Node for $node_name {}
    };
}

pub trait Node: std::fmt::Debug {}
node!(Literal, t: Token);
