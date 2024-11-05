use crate::{
    error::Error,
    rules::Rule,
    types::{Keyword, Token, Type},
};
use nodes::{Explain, Literal, Node, Vacuum};

mod nodes;
mod tests;

pub struct Parser<'a> {
    pos: usize,
    tokens: Vec<Token>,
    name: &'a str,
    pub errors: Vec<Error>,
}

/// Function naming directly corresponds to the sqlite3 documentation of sql syntax.
///
/// ## See:
///
/// - https://www.sqlite.org/lang.html
/// - https://www.sqlite.org/lang_expr.html
impl<'a> Parser<'a> {
    pub fn new(tokens: Vec<Token>, name: &'a str) -> Parser<'a> {
        Parser {
            pos: 0,
            name,
            tokens,
            errors: vec![],
        }
    }

    fn cur(&self) -> Option<&Token> {
        self.tokens.get(self.pos)
    }

    fn err(&self, msg: &str, note: &str, start: &Token, rule: Rule) -> Error {
        Error {
            file: self.name.to_string(),
            line: start.line,
            rule,
            note: note.into(),
            msg: msg.into(),
            start: start.start,
            end: match self.cur() {
                Some(tok) => tok.end,
                None => start.start,
            },
            doc_url: None,
        }
    }

    fn is_eof(&self) -> bool {
        self.pos >= self.tokens.len()
    }

    fn advance(&mut self) {
        if !self.is_eof() {
            self.pos += 1
        }
    }

    fn is(&self, t: Type) -> bool {
        self.cur().map_or(false, |tok| tok.ttype == t)
    }

    /// checks if type of current token is equal to t, otherwise pushes an error, advances either way
    fn consume(&mut self, t: Type) {
        let tt = t.clone();
        if !self.is(tt) {
            let cur = match self.cur() {
                None => {
                    let last = self.tokens.get(self.pos - 1).unwrap();
                    &Token {
                        ttype: Type::Eof,
                        start: last.end,
                        end: last.end,
                        line: last.line,
                    }
                }
                Some(c) => c,
            };
            let mut err = self.err(
                match cur.ttype {
                    Type::Eof => "Unexpected End of input",
                    _ => "Unexpected Token",
                },
                &format!("Wanted {:?}, got {:?}", t, cur.ttype),
                cur,
                Rule::Syntax,
            );
            if t == Type::Semicolon {
                err.msg = "Missing semicolon".into();
                err.note = "Terminate statements with ';'".into();
                err.rule = Rule::Semicolon;
            }
            err.doc_url = Some("https://www.sqlite.org/syntax/sql-stmt.html");
            self.errors.push(err);
        }
        self.advance(); // we advance either way to keep the parser error resistant
    }

    fn next_is(&self, t: Type) -> bool {
        self.tokens
            .get(self.pos + 1)
            .map_or(false, |tok| tok.ttype == t)
    }

    pub fn parse(&mut self) -> Vec<Option<Box<dyn Node>>> {
        self.sql_stmt_list()
    }

    /// see: https://www.sqlite.org/syntax/sql-stmt-list.html
    fn sql_stmt_list(&mut self) -> Vec<Option<Box<dyn Node>>> {
        let mut r = vec![];
        while !self.is_eof() {
            let stmt = self.sql_stmt_prefix();
            if stmt.is_some() {
                r.push(stmt);
            }
            self.consume(Type::Semicolon);
        }
        r
    }

    fn sql_stmt_prefix(&mut self) -> Option<Box<dyn Node>> {
        match self.cur()?.ttype {
            Type::Keyword(Keyword::EXPLAIN) => {
                let mut e = Explain {
                    t: self.cur()?.clone(),
                    child: None,
                };
                self.advance(); // skip EXPLAIN

                // path for EXPLAIN->QUERY->PLAN
                if self.is(Type::Keyword(Keyword::QUERY)) {
                    self.consume(Type::Keyword(Keyword::QUERY));
                    self.consume(Type::Keyword(Keyword::PLAN));
                } // else path is EXPLAIN->*_stmt

                e.child = self.sql_stmt();
                Some(Box::new(e))
            }
            _ => self.sql_stmt(),
        }
    }

    /// see: https://www.sqlite.org/syntax/sql-stmt.html
    fn sql_stmt(&mut self) -> Option<Box<dyn Node>> {
        // TODO:
        match self.cur()?.ttype {
            Type::Keyword(Keyword::VACUUM) => self.vacuum_stmt(),

            // explicitly disallowing literals at this point
            Type::String(_)
            | Type::Number(_)
            | Type::Blob(_)
            | Type::Keyword(Keyword::NULL)
            | Type::Boolean(_)
            | Type::Keyword(Keyword::CURRENT_TIME)
            | Type::Keyword(Keyword::CURRENT_DATE)
            | Type::Keyword(Keyword::CURRENT_TIMESTAMP) => {
                self.errors.push(self.err(
                    "Unexpected Literal",
                    &format!(
                        "No top level literals, such as {:?} allowed.",
                        self.cur()?.ttype
                    ),
                    self.cur()?,
                    Rule::Syntax,
                ));
                self.advance();
                None
            }
            _ => {
                self.errors.push(self.err(
                    "Unimplemented",
                    &format!(
                        "sqleibniz can not yet analyse the token {:?}",
                        self.cur()?.ttype
                    ),
                    self.cur()?,
                    Rule::Unimplemented,
                ));
                self.advance();
                None
            }
        }
    }

    /// https://www.sqlite.org/lang_vacuum.html
    fn vacuum_stmt(&mut self) -> Option<Box<dyn Node>> {
        let mut v = Vacuum {
            t: self.cur()?.clone(),
            schema_name: None,
            filename: None,
        };
        self.consume(Type::Keyword(Keyword::VACUUM));

        match self.cur()?.ttype {
            Type::Semicolon | Type::Ident(_) | Type::Keyword(Keyword::INTO) => {}
            _ => {
                let mut err = self.err(
                    "Unexpected Token",
                    &format!(
                        "Wanted {:?} with {:?} or {:?} for VACUUM stmt, got {:?}",
                        Type::Keyword(Keyword::INTO),
                        Type::String("<filename>".to_string()),
                        Type::Ident("<schema_name>".to_string()),
                        self.cur()?.ttype.clone()
                    ),
                    &self.cur()?.clone(),
                    Rule::Syntax,
                );
                err.doc_url = Some("https://www.sqlite.org/lang_vacuum.html");
                self.errors.push(err);
                self.advance(); // skip error_token
            }
        }

        // first path
        if let Type::Semicolon = self.cur()?.ttype {
            return Some(Box::new(v));
        }

        // if schema_name is specified
        if let Type::Ident(_) = self.cur()?.ttype {
            v.schema_name = Some(self.cur()?.clone());
            self.advance(); // skip schema_name
        }

        // if INTO keyword is given is specified
        if let Type::Keyword(Keyword::INTO) = self.cur()?.ttype {
            self.advance(); // skip INTO
            if let Type::String(_) = self.cur()?.ttype {
                v.filename = Some(self.cur()?.clone());
            } else {
                let mut err = self.err(
                    "Unexpected Token",
                    &format!(
                        "Wanted {:?} for VACUUM stmt with {:?}, got {:?}",
                        Type::String("<filename>".to_string()),
                        Type::Keyword(Keyword::INTO),
                        self.cur()?.ttype.clone()
                    ),
                    &self.cur()?.clone(),
                    Rule::Syntax,
                );
                err.doc_url = Some("https://www.sqlite.org/lang_vacuum.html");
                self.errors.push(err);
            }
            self.advance(); // skip filename or error token
        }

        Some(Box::new(v))
    }

    /// see: https://www.sqlite.org/syntax/literal-value.html
    fn literal_value(&mut self) -> Option<Box<dyn Node>> {
        let cur = self.cur()?;
        match cur.ttype {
            Type::String(_)
            | Type::Number(_)
            | Type::Blob(_)
            | Type::Keyword(Keyword::NULL)
            | Type::Boolean(_)
            | Type::Keyword(Keyword::CURRENT_TIME)
            | Type::Keyword(Keyword::CURRENT_DATE)
            | Type::Keyword(Keyword::CURRENT_TIMESTAMP) => {
                let s: Option<Box<dyn Node>> = Some(Box::new(Literal { t: cur.clone() }));
                // skipping over the current character
                self.advance();
                s
            }
            _ => {
                let mut err = self.err("Unexpected Token", &format!("Wanted a literal (any of number,string,blob,null,true,false,CURRENT_TIME,CURRENT_DATE,CURRENT_DATE), got {:?}", cur.ttype),cur, Rule::Syntax);
                err.doc_url = Some("https://www.sqlite.org/syntax/literal-value.html");
                self.errors.push(err);
                None
            }
        }
    }
}
