use crate::error::{Error, ErrorType};
use crate::types::{Token, Type};

pub struct Lexer<'a> {
    pos: usize,
    line: usize,
    name: String,
    source: &'a Vec<u8>,
    pub errors: Vec<Error>,
}

impl Lexer<'_> {
    pub fn init(source: &'_ Vec<u8>, name: String) -> Lexer<'_> {
        Lexer {
            pos: 0,
            line: 0,
            name,
            source,
            errors: vec![],
        }
    }

    fn err(&self, msg: &str, note: &str, start: usize) -> Error {
        Error {
            file: self.name.clone(),
            line: self.line,
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

    fn is_ident(&self) -> bool {
        match self.source.get(self.pos) {
            Some(cc) => match *cc as char {
                'a'..='z' => true,
                'A'..='Z' => true,
                '_' => true,
                _ => false,
            },
            _ => false,
        }
    }

    fn is(&self, c: char) -> bool {
        match self.source.get(self.pos) {
            Some(cc) => *cc as char == c,
            _ => false,
        }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.source.len()
    }

    fn is_sqlite_num(&self) -> bool {
        match self.source.get(self.pos) {
            Some(cc) => match *cc as char {
                // hexadecimal
                'x' | 'X' => true,
                // sqlite allows for separating numbers by _
                '_' => true,
                _ => false,
            },
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

        while !self.is_eof() {
            let cc = match self.source.get(self.pos) {
                Some(cc) => *cc,
                _ => break,
            } as char;
            match cc {
                // comments, see: https://www.sqlite.org/lang_comment.html
                '/' => {
                    if self.next_equals('*') {
                        while !self.is_eof() {
                            self.pos += 1;
                            if self.is('\n') {
                                self.line += 1;
                            } else if self.is('*') && self.next_equals('/') {
                                break;
                            }
                        }
                    }
                }
                '\n' => {
                    self.line += 1;
                    self.pos += 1;
                }
                // comments, see: https://www.sqlite.org/lang_comment.html
                '-' => {
                    if self.next_equals('-') {
                        while !self.is_eof() {
                            self.pos += 1;
                            if self.is('\n') {
                                self.line += 1;
                                break;
                            }
                        }
                    }
                }
                // string, see: https://www.sqlite.org/lang_expr.html#literal_values_constants_
                '\'' => {
                    let start = self.pos;
                    while !self.is_eof() {
                        self.pos += 1;
                        if self.is('\n') || self.is_eof() {
                            self.line += 1;
                            self.errors.push(self.err(
                                &format!("Unterminated String in '{}'", self.name),
                                "Consider adding a \"'\" at the end of this string",
                                start,
                            ));
                            break;
                        } else if self.is('\'') {
                            r.push(Token {
                                ttype: Type::String(
                                    String::from_utf8(
                                        self.source
                                            // +1 to skip the ' from the start of the string
                                            .get(start + 1..self.pos)
                                            .unwrap_or_default()
                                            .to_vec(),
                                    )
                                    .unwrap_or_default(),
                                ),
                                end: self.pos - 1,
                                start,
                            });
                            self.pos += 1;
                            break;
                        }
                    }
                }
                // number, see above
                '0'..='9' | '.' => {
                    todo!("Numbers arent yet implemented")
                }
                // blobs, see above
                'X' | 'x' => {
                    todo!("Blobs arent yet implemented")
                }
                // identifiers / keywords: https://www.sqlite.org/lang_keywords.html
                _ => {}
            }
            self.pos += 1;
        }

        if r.len() == 0 && self.errors.len() == 0 {
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
