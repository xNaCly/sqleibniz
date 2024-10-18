use crate::types::Token;

pub trait Node: std::fmt::Debug {}

// TODO: write a macro to automate this
// node!(Literal, t: Token) -> below

#[derive(Debug)]
pub struct Literal {
    pub t: Token,
}
impl Node for Literal {}
