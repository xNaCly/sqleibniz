use crate::{error::Error, rules::Rule, types::Token};
use std::fmt::Debug;

pub trait Node: Debug {}

#[derive(Debug)]
pub struct Literal<'a> {
    t: &'a Token,
}
impl Node for Literal<'_> {}

pub struct Parser {
    pos: usize,
    tokens: Vec<Token>,
    pub errors: Vec<Error>,
}

impl Parser {
    pub fn init(tokens: Vec<Token>) -> Parser {
        Parser {
            pos: 0,
            tokens,
            errors: vec![],
        }
    }

    fn cur(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn err(&self, msg: &str, note: &str, start: &Token, rule: Rule) -> Error {
        Error {
            file: String::from("i have to keep the file somewhere"),
            line: start.line,
            rule,
            note: note.into(),
            msg: msg.into(),
            start: start.start,
            end: self.cur().end,
            doc_url: None,
        }
    }

    pub fn parse(&mut self) -> Box<dyn Node + '_> {
        self.errors.push(self.err(
            "Parser is not implemented",
            "nothing in the parser yet",
            &self.tokens[0],
            Rule::Unimplemented,
        ));
        dbg!(&self.tokens[0]);
        let l = Literal { t: self.cur() };
        Box::new(l)
    }
}
