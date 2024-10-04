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

    fn next_equals(&mut self, c: char) -> bool {
        match self.source.get(self.pos + 1) {
            Some(cc) => *cc == c as u8,
            _ => false,
        }
    }

    fn equals(self, c: char) -> bool {
        match self.source.get(self.pos) {
            Some(cc) => *cc as char == c,
            _ => false,
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

        loop {
            let cc = match self.source.get(self.pos) {
                Some(cc) => *cc,
                _ => break,
            } as char;
            match cc {
                '/' => {
                    if self.next_equals('*') {
                        loop {
                            self.pos += 1;
                            if let Some(cc) = self.source.get(self.pos) {
                                if *cc as char == '*' {
                                    if let Some(cc) = self.source.get(self.pos + 1) {
                                        if *cc as char == '/' {
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    }
                }
                '-' => {
                    if self.next_equals('-') {
                        loop {
                            self.pos += 1;
                            if let Some(cc) = self.source.get(self.pos) {
                                if *cc as char == '\n' {
                                    break;
                                }
                            };
                        }
                    }
                }
                _ => {
                    // TODO: keywords and integers
                    // let start = self.pos;
                    // while self.equals() {}
                }
            }
            self.pos += 1;
        }

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
