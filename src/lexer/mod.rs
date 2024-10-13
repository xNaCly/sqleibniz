use crate::error::Error;
use crate::rules::Rule;
use crate::types::{Keyword, Token, Type};

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

    fn is_sqlite_num(&self, c: char) -> bool {
        match c {
            // hexadecimal
            'x' | 'X' => true,
            // sqlite allows for separating numbers by _
            '_' => true,
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
                        if self.is('\n') || self.is_eof() {
                            let mut err = self.err(
                                &format!("Unterminated String in '{}'", self.name),
                                "Consider adding a \"'\" at the end of this string",
                                line_start,
                                Rule::UnterminatedString,
                            );
                            err.end = end + 1;
                            err.line = line;
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
                    if self.is('.') {
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

                    self.errors.push(self.err(
                        "Unimplemented: Numbers",
                        "Numbers arent yet implemented",
                        self.line_pos,
                        Rule::Unimplemented,
                    ));
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
                    while self.is_ident(self.cur()) {
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
                    self.errors.push(self.err(
                        &format!("Unknown character '{}'", cur),
                        &format!(
                            "character (ascii: {:#?}, decimal: {}, hex: {:#x}) is unknown at this time",
                            cur, cur as u8, cur as u8
                        ),
                        self.line_pos,
                        Rule::UnknownCharacter,
                    ));
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
