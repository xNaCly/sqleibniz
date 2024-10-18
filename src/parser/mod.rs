use crate::{error::Error, rules::Rule, types::Token};
use nodes::{Literal, Node};

mod nodes;

pub struct Parser {
    pos: usize,
    tokens: Vec<Token>,
    name: String,
    pub errors: Vec<Error>,
}

impl Parser {
    pub fn new(tokens: Vec<Token>, name: String) -> Parser {
        Parser {
            pos: 0,
            name,
            tokens,
            errors: vec![],
        }
    }

    fn cur(&self) -> &Token {
        &self.tokens[self.pos]
    }

    fn err(&self, msg: &str, note: &str, start: &Token, rule: Rule) -> Error {
        Error {
            file: self.name.clone(),
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
