use std::f64;

use crate::error::{self, Error, ImprovedLine};
use crate::rules::Rule;
use crate::types::{Keyword, Token, Type};

mod tests;

pub struct Lexer<'a> {
    pos: usize,
    line: usize,
    line_pos: usize,
    name: &'a str,
    source: &'a Vec<u8>,
    pub errors: Vec<Error>,
}

impl<'a> Lexer<'a> {
    pub fn new(source: &'a Vec<u8>, name: &'a str) -> Lexer<'a> {
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
            improved_line: None,
            file: self.name.to_string(),
            line: self.line,
            rule,
            note: note.into(),
            msg: msg.into(),
            start,
            end: self.line_pos,
            doc_url: None,
        }
    }

    fn next_is(&mut self, c: char) -> bool {
        self.source
            .get(self.pos + 1)
            .is_some_and(|cc| *cc == c as u8)
    }

    fn is_ident(&self, c: char) -> bool {
        matches!(c, 'a'..='z' | 'A'..='Z' | '_' | '0'..='9')
    }

    fn is(&self, c: char) -> bool {
        self.source.get(self.pos).is_some_and(|cc| *cc as char == c)
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.source.len()
    }

    /// Specifically matches https://www.sqlite.org/syntax/numeric-literal.html
    fn is_sqlite_num(&self) -> bool {
        matches!(self.cur(), 
                 // exponent notation with +-
                 '+' | '-' |
                 // sqlite allows for separating numbers by _
                 '_' |
                 // floating point
                 '.' |
                 // hexadecimal
                 'a'..='f' | 'A'..='F' |
                 // decimal
                 '0'..='9')
    }

    fn cur(&self) -> char {
        self.source[self.pos] as char
    }

    fn next(&self) -> Option<char> {
        self.source.get(self.pos + 1).map(|c| *c as char)
    }

    fn single(&self, ttype: Type) -> Token {
        Token {
            ttype,
            start: self.line_pos,
            end: self.line_pos,
            line: self.line,
        }
    }

    /// progresses in the input until ',\n or EOF are hit.
    fn string(&mut self) -> Result<Token, error::Error> {
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
                err.doc_url =
                    Some("https://www.sqlite.org/lang_expr.html#literal_values_constants_");
                err.improved_line = Some(ImprovedLine{
                    snippet: "'",
                    start: err.end,
                });
                return Err(err);
            } else if self.is('\'') {
                return Ok(Token {
                    line: self.line,
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
                    end: end + 1,
                    start: line_start,
                });
            }
        }
        Err(self.err("Impossible case", "", self.line_pos, Rule::Unimplemented))
    }

    pub fn run(&mut self) -> Vec<Token> {
        let mut r = vec![];
        if self.source.is_empty() {
            self.errors.push(self.err(
                "No content found in source file",
                &format!("consider adding statements to '{}'", self.name),
                0,
                Rule::NoContent,
            ));
            return vec![];
        };

        while !self.is_eof() {
            match self.cur() {
                // skipping whitespace
                '\t' | '\r' | ' ' | '\n' => {}
                // comments, see: https://www.sqlite.org/lang_comment.html
                '/' => {
                    if self.next_is('*') {
                        while !self.is_eof() {
                            self.advance();
                            if self.is('*') && self.next_is('/') {
                                break;
                            }
                        }
                    }
                }
                // comments, see: https://www.sqlite.org/lang_comment.html
                '-' => {
                    if self.next_is('-') {
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
                    match self.string() {
                        Ok(str_tok) => r.push(str_tok),
                        Err(err) => self.errors.push(err),
                    }
                }
                '*' => r.push(self.single(Type::Asteriks)),
                ';' => r.push(self.single(Type::Semicolon)),
                ',' => r.push(self.single(Type::Comma)),
                '%' => r.push(self.single(Type::Percent)),
                // numbers, see: https://www.sqlite.org/lang_expr.html#literal_values_constants_
                '0'..='9' | '.' => {
                    // only '.', with no digit following it is an indexing operation
                    // check if next is not e/E, because these are used as scientifc notation
                    // in floating point numbers
                    if self.is('.') 
                        && !(self.next_is('e') || self.next_is('E')) 
                        && self.next().is_some_and(|c| matches!(c, 'a'..='z' | 'A'..='Z' | '_'))
                        {
                            r.push(Token {
                                ttype: Type::Dot,
                                line: self.line,
                                start: self.line_pos,
                                end: self.line_pos,
                            });
                            self.advance();
                            continue;
                        };

                    let line_start = self.line_pos;

                    // hexadecimal number
                    let is_hex = if self.is('0') && (self.next_is('x') || self.next_is('X'))
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
                        .iter()
                        .filter_map(|&u| match u as char {
                            '_' => None,
                            _ => Some(u as char),
                        })
                        .collect::<String>();

                    if is_hex {
                        match i64::from_str_radix(&str, 16) {
                            Ok(number) => {
                                r.push(Token {
                                    line: self.line,
                                    ttype: Type::Number(number as f64),
                                    start: line_start,
                                    end: self.line_pos,
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
                                    line: self.line,
                                    ttype: Type::Number(number),
                                    start: line_start,
                                    end: self.line_pos,
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
                    // this skips the advance at the bottom of the while loop
                    continue;
                }
                // blobs, see above
                'X' | 'x' => {
                    let line_start = self.line_pos;
                    let line = self.line;
                    if self.next_is('\'') {
                        self.advance(); // skip X
                        let result = self.string();
                        if let Ok(str_tok) = &result {
                            match &str_tok.ttype {
                                Type::String(str) => {
                                    let mut had_bad_hex = false;
                                    for (idx, c) in str.chars().enumerate() {
                                        match c {
                                            'a'..='f' => {}
                                            'A'..='F' => {}
                                            '0'..='9' => {}
                                            _ => {
                                                let mut err = self.err("Bad blob data", &format!("a Blob is hexadecimal data, '{}' is not a valid hexadecimal digit (a..=f, A..=F, 0..=9)", c), line_start+2+idx, Rule::InvalidBlob);
                                                err.end = line_start + 2 + idx;
                                                err.doc_url = Some("https://www.sqlite.org/lang_expr.html#literal_values_constants_");
                                                self.errors.push(err);
                                                had_bad_hex = true;
                                                break;
                                            }
                                        }
                                    }
                                    if !had_bad_hex {
                                        r.push(Token {
                                            line,
                                            ttype: Type::Blob(str.as_bytes().to_vec()),
                                            start: str_tok.start,
                                            end: str_tok.end,
                                        });
                                    }
                                }
                                _ => panic!("Impossible branch"),
                            }
                        } else {
                            let mut err = self.err(
                                "Unterminated blob string",
                                "a Blob is hexadecimal data prefixed with X' and postfixed with ', you forgot the closing '",
                                line_start,
                                Rule::InvalidBlob,
                            );
                            err.line = line;
                            err.doc_url = Some(
                                "https://www.sqlite.org/lang_expr.html#literal_values_constants_",
                            );
                            self.errors.push(err);
                        }
                    } else {
                        let mut err = self.err(
                            "Malformed blob",
                            "a Blob is hexadecimal data prefixed with X' and postfixed with '",
                            self.line_pos,
                            Rule::InvalidBlob,
                        );
                        err.doc_url =
                            Some("https://www.sqlite.org/lang_expr.html#literal_values_constants_");
                        self.errors.push(err);
                    }
                }
                // identifiers / keywords: https://www.sqlite.org/lang_keywords.html
                'a'..='z' | 'A'..='Z' | '_' => {
                    let start = self.pos;
                    let line_start = self.line_pos;
                    while !self.is_eof() && self.is_ident(self.cur()) {
                        self.advance();
                    }
                    let chars = self.source
                            .get(start..self.pos)
                            .unwrap_or_default()
                            .to_vec();
                    let ident = String::from_utf8(chars).unwrap_or_default();
                    let t: Type = if let Some(keyword) = Keyword::from_str(ident.as_str()) {
                        Type::Keyword(keyword)
                    } else if ident.to_lowercase() == "true" || ident.to_lowercase() == "false" {
                        Type::Boolean(ident.to_lowercase() == "true")
                    } else {
                        Type::Ident(ident.clone())
                    };
                    r.push(Token {
                        line: self.line,
                        ttype: t,
                        start: line_start,
                        end: self.line_pos,
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

        if r.is_empty() && self.errors.is_empty() {
            self.errors.push(self.err(
                "No statements found in source file",
                &format!("consider adding statements to '{}'", self.name),
                0,
                Rule::NoStatements,
            ));
            return vec![];
        }
        r
    }
}
