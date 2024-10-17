use std::f64;

use crate::error::Error;
use crate::rules::Rule;
use crate::types::{Keyword, Token, Type};

mod tests;

pub struct Lexer<'a> {
    pos: usize,
    line: usize,
    line_pos: usize,
    name: String,
    source: &'a Vec<u8>,
    pub errors: Vec<Error>,
}

impl Lexer<'_> {
    pub fn init(source: &'_ Vec<u8>, name: String) -> Lexer<'_> {
        Lexer {
            pos: 0,
            line: 0,
            line_pos: 0,
            name,
            source,
            errors: vec![],
        }
    }

    fn advance(&mut self) {
        if self.is('\n') {
            self.line += 1;
            self.line_pos = 0;
        } else {
            self.line_pos += 1;
        }
        self.pos += 1;
    }

    fn err(&self, msg: &str, note: &str, start: usize, rule: Rule) -> Error {
        Error {
            file: self.name.clone(),
            line: self.line,
            rule,
            note: note.into(),
            msg: msg.into(),
            start,
            end: self.line_pos,
            doc_url: None,
        }
    }

    fn next_equals(&mut self, c: char) -> bool {
        match self.source.get(self.pos + 1) {
            Some(cc) => *cc == c as u8,
            _ => false,
        }
    }

    fn is_ident(&self, c: char) -> bool {
        match c {
            'a'..='z' => true,
            'A'..='Z' => true,
            '_' => true,
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

    /// Specifically matches https://www.sqlite.org/syntax/numeric-literal.html
    fn is_sqlite_num(&self) -> bool {
        match self.cur() {
            // exponent notation with +-
            '+' | '-' => true,
            // sqlite allows for separating numbers by _
            '_' => true,
            // floating point
            '.' => true,
            // hexadecimal
            'a'..='f' => true,
            'A'..='F' => true,
            // decimal
            '0'..='9' => true,
            _ => false,
        }
    }

    fn cur(&self) -> char {
        return self.source[self.pos] as char;
    }

    fn next(&self) -> Option<char> {
        match self.source.get(self.pos + 1) {
            Some(c) => Some(*c as char),
            _ => None,
        }
    }

    fn single(&self, ttype: Type) -> Token {
        Token {
            ttype,
            start: self.pos,
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
                Rule::NoContent,
            ));
            return vec![];
        };

        while !self.is_eof() {
            let cc = match self.source.get(self.pos) {
                Some(cc) => *cc,
                _ => break,
            } as char;
            match cc {
                // skipping whitespace
                '\t' | '\r' | ' ' | '\n' => {}
                // comments, see: https://www.sqlite.org/lang_comment.html
                '/' => {
                    if self.next_equals('*') {
                        while !self.is_eof() {
                            self.advance();
                            if self.is('*') && self.next_equals('/') {
                                break;
                            }
                        }
                    }
                }
                // comments, see: https://www.sqlite.org/lang_comment.html
                '-' => {
                    if self.next_equals('-') {
                        while !self.is_eof() {
                            self.advance();
                            if self.is('\n') {
                                break;
                            }
                        }
                    }
                }
                // string, see: https://www.sqlite.org/lang_expr.html#literal_values_constants_
                '\'' => {
                    let start = self.pos;
                    let line_start = self.line_pos;
                    while !self.is_eof() {
                        let end = self.line_pos;
                        let line = self.line;
                        self.advance();
                        if self.is_eof() || self.is('\n') {
                            let mut err = self.err(
                                &format!("Unterminated String in '{}'", self.name),
                                "Consider adding a \"'\" at the end of this string",
                                line_start,
                                Rule::UnterminatedString,
                            );
                            err.end = end + 1;
                            err.line = line;
                            err.doc_url = Some(
                                "https://www.sqlite.org/lang_expr.html#literal_values_constants_",
                            );
                            self.errors.push(err);
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
                            break;
                        }
                    }
                }
                '*' => r.push(self.single(Type::Asteriks)),
                ';' => r.push(self.single(Type::Semicolon)),
                ',' => r.push(self.single(Type::Comma)),
                '%' => r.push(self.single(Type::Percent)),
                // numbers, see: https://www.sqlite.org/lang_expr.html#literal_values_constants_
                '0'..='9' | '.' => {
                    // only '.', with no digit following it is an indexing operation
                    if self.is('.') && 
                        // check if next is not e/E, because these are used as scientifc notation
                        // in floating point numbers
                        !(self.next_equals('e') || self.next_equals('E')) {
                        let next = self.next();
                        if next.is_some() && self.is_ident(next.unwrap()) {
                            r.push(Token {
                                ttype: Type::Dot,
                                start: self.pos,
                                end: self.pos,
                            });
                            self.advance();
                            continue;
                        };
                    }

                    let line_start = self.line_pos;

                    // hexadecimal number
                    let is_hex = if self.is('0') && (self.next_equals('x') || self.next_equals('X'))
                    {
                        self.advance();
                        self.advance();
                        true
                    } else {
                        false
                    };

                    // number state machine
                    let start = self.pos;
                    while !self.is_eof() && self.is_sqlite_num() {
                        self.advance();
                    }

                    let str = self
                        .source
                        .get(start..self.pos)
                        .unwrap_or_default()
                        .into_iter()
                        .filter_map(|&u| match u as char {
                            '_' => None,
                            _ => Some(u as char),
                        })
                        .collect::<String>();

                    if is_hex {
                        match i64::from_str_radix(&str, 16) {
                            Ok(number) => {
                                r.push(Token {
                                    ttype: Type::Number(number as f64),
                                    start: self.pos,
                                    end: self.pos,
                                });
                            }
                            Err(error) => {
                                let mut err = self.err(
                                    &format!("Bad hexadecimal numeric literal: '0x{}'", str),
                                    &error.to_string(),
                                    line_start,
                                    Rule::InvalidNumericLiteral,
                                );
                                err.doc_url =
                                    Some("https://www.sqlite.org/syntax/numeric-literal.html");
                                self.errors.push(err);
                            }
                        };
                    } else {
                        match str.parse::<f64>() {
                            Ok(number) => {
                                r.push(Token {
                                    ttype: Type::Number(number as f64),
                                    start: self.pos,
                                    end: self.pos,
                                });
                            }
                            Err(error) => {
                                let mut err = self.err(
                                    &format!("Bad numeric literal: '{}'", str),
                                    &error.to_string(),
                                    line_start,
                                    Rule::InvalidNumericLiteral,
                                );
                                err.doc_url =
                                    Some("https://www.sqlite.org/syntax/numeric-literal.html");
                                self.errors.push(err);
                            }
                        };
                    };
                }
                // blobs, see above
                'X' | 'x' => {
                    self.errors.push(self.err(
                        "Unimplemented: Blobs",
                        "Blobs arent yet implemented",
                        self.line_pos,
                        Rule::Unimplemented,
                    ));
                }
                // identifiers / keywords: https://www.sqlite.org/lang_keywords.html
                'a'..='z' | 'A'..='Z' | '_' => {
                    let start = self.pos;
                    while !self.is_eof() && self.is_ident(self.cur()) {
                        self.advance();
                    }
                    let ident = String::from_utf8(
                        self.source
                            .get(start..self.pos)
                            .unwrap_or_default()
                            .to_vec(),
                    )
                    .unwrap_or_default();
                    let t: Type;
                    if let Some(keyword) = Keyword::from_str(ident.as_str()) {
                        t = Type::Keyword(keyword);
                    } else {
                        t = Type::Ident(ident.clone());
                    }
                    r.push(Token {
                        ttype: t,
                        start,
                        end: self.pos,
                    });
                    continue;
                }
                _ => {
                    let cur = self.cur();
                    let mut err = self.err(
                        &format!("Unknown character '{}'", cur),
                        &format!(
                            "character (ascii: {:#?}, decimal: {}, hex: {:#x})",
                            cur, cur as u8, cur as u8
                        ),
                        self.line_pos,
                        Rule::UnknownCharacter,
                    );
                    err.doc_url = Some("https://www.sqlite.org/syntax/expr.html");
                    self.errors.push(err);
                }
            }
            self.advance();
        }

        if r.len() == 0 && self.errors.len() == 0 {
            self.errors.push(self.err(
                "No statements found in source file",
                &format!("consider adding statements to '{}'", self.name),
                0,
                Rule::NoStatements,
            ));
            return vec![];
        }
        return r;
    }
}
