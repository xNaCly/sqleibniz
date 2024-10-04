use crate::error::{Error, ErrorType};
use crate::types::Token;

pub struct Lexer<'a> {
    pos: usize,
    name: String,
    source: &'a Vec<u8>,
    pub errors: Vec<Error>,
}

impl<'a> Lexer<'a> {
    pub fn init(source: &'a Vec<u8>, name: String) -> Lexer<'a> {
        Lexer {
            pos: 0,
            name,
            source,
            errors: vec![],
        }
    }

    fn err(&self, msg: &str, note: &str, start: usize) -> Error {
        Error {
            etype: ErrorType::SyntaxError,
            note: note.into(),
            msg: msg.into(),
            start,
            end: self.pos,
        }
    }

    pub fn run(&mut self) -> Vec<Token> {
        let mut r = vec![];
        if self.source.len() == 0 {
            self.errors.push(self.err(
                "No content found in source file",
                &format!("consider adding statements to '{}'", self.name),
                0,
            ));
            return vec![];
        };
        if r.len() == 0 {
            self.errors.push(self.err(
                "No statements found in source file",
                &format!("consider adding statements to '{}'", self.name),
                0,
            ));
            return vec![];
        }
        return r;
    }
}
