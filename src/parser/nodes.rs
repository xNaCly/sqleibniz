use crate::types::Token;
use std::fmt::Debug;

pub trait Node: Debug {}

#[derive(Debug)]
pub struct Literal<'a> {
    pub t: &'a Token,
}
impl Node for Literal<'_> {}
