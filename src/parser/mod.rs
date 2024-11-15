use crate::{
    error::{Error, ImprovedLine},
    rules::Rule,
    types::{Keyword, Token, Type},
};
use nodes::{Begin, Commit, Explain, Literal, Node, Rollback, Vacuum};

mod nodes;
mod tests;

pub struct Parser<'a> {
    pos: usize,
    tokens: Vec<Token>,
    name: &'a str,
    pub errors: Vec<Error>,
}

/// wrap $expr in Some(Box::new($expr))
macro_rules! some_box {
    ($expr:expr) => {
        Some(Box::new($expr))
    };
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
            improved_line: None,
            file: self.name.to_string(),
            line: start.line,
            rule,
            note: note.into(),
            msg: msg.into(),
            start: start.start,
            end: self.cur().map_or(start.start, |tok| tok.end),
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

    fn skip_until_semicolon(&mut self) {
        while !self.is(Type::Semicolon) {
            self.advance();
        }
    }

    /// if current token in t advance, otherwise return false; finally advance
    fn matches_any(&mut self, t: Vec<Type>) -> Option<Token> {
        if let Some(cur) = &self.cur() {
            if t.contains(&cur.ttype) {
                let t = (*cur).clone();
                self.advance();
                return Some(t);
            }
            return None;
        }
        None
    }

    /// if current token in t advance, otherwise return false and push error; finally advance
    fn matches_one(&mut self, t: Vec<Type>) -> bool {
        if let Some(cur) = &self.cur() {
            if !t.contains(&cur.ttype) {
                self.errors.push(self.err(
                    "Unexpected Token",
                    &format!("Wanted any of {:?}, got {:?}", t, cur.ttype),
                    cur,
                    Rule::Syntax,
                ));
                return false;
            }
            self.advance();
            return true;
        }
        false
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
                err.note.push_str(", terminate statements with ';'");
                err.rule = Rule::Semicolon;
                err.improved_line = Some(ImprovedLine {
                    snippet: ";",
                    start: cur.end,
                });
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
            if let Some(Token {
                ttype: Type::InstructionExpect,
                ..
            }) = self.cur()
            {
                // skip all token until the statement ends
                while !self.is(Type::Semicolon) {
                    self.advance();
                }
                // skip ';'
                self.advance();
                continue;
            }
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
                // skip EXPLAIN
                self.advance();

                // path for EXPLAIN->QUERY->PLAN
                if self.is(Type::Keyword(Keyword::QUERY)) {
                    self.advance();
                    self.consume(Type::Keyword(Keyword::PLAN));
                }

                // else path is EXPLAIN->*_stmt
                e.child = self.sql_stmt();
                some_box!(e)
            }
            _ => self.sql_stmt(),
        }
    }

    /// see: https://www.sqlite.org/syntax/sql-stmt.html
    fn sql_stmt(&mut self) -> Option<Box<dyn Node>> {
        match self.cur()?.ttype {
            Type::Keyword(Keyword::VACUUM) => self.vacuum_stmt(),
            Type::Keyword(Keyword::BEGIN) => self.begin_stmt(),
            Type::Keyword(Keyword::COMMIT) | Type::Keyword(Keyword::END) => self.commit_stmt(),
            Type::Keyword(Keyword::ROLLBACK) => self.rollback_stmt(),
            Type::Semicolon => {
                self.errors.push(self.err(
                    "Unexpected Token",
                    "Semicolon makes no sense at this point",
                    self.cur()?,
                    Rule::Syntax,
                ));
                self.advance();
                None
            }

            // explicitly disallowing literals at this point: results in clearer and more
            // understandable error messages
            Type::String(_)
            | Type::Number(_)
            | Type::Blob(_)
            | Type::Keyword(Keyword::NULL)
            | Type::Boolean(_)
            | Type::Keyword(Keyword::CURRENT_TIME)
            | Type::Keyword(Keyword::CURRENT_DATE)
            | Type::Keyword(Keyword::CURRENT_TIMESTAMP) => {
                let mut err = self.err(
                    "Unexpected Literal",
                    &format!("Literal {:?} disallowed at this point.", self.cur()?.ttype),
                    self.cur()?,
                    Rule::Syntax,
                );
                err.doc_url = Some("https://www.sqlite.org/syntax/sql-stmt.html");
                self.errors.push(err);
                self.advance();
                None
            }
            _ => {
                self.errors.push(self.err(
                    "Unimplemented",
                    &format!(
                        "sqleibniz can not yet analyse the token {:?}, skipping ahead to next statement",
                        self.cur()?.ttype
                    ),
                    self.cur()?,
                    Rule::Unimplemented,
                ));
                self.skip_until_semicolon();
                None
            }
        }
    }

    /// https://www.sqlite.org/syntax/rollback-stmt.html
    fn rollback_stmt(&mut self) -> Option<Box<dyn Node>> {
        let mut rollback = Rollback {
            t: self.cur()?.clone(),
            save_point: None,
        };
        self.advance();

        match self.cur()?.ttype {
            Type::Keyword(Keyword::TRANSACTION) | Type::Keyword(Keyword::TO) | Type::Semicolon => {}
            _ => {
                let mut err = self.err(
                    "Unexpected Token",
                    &format!(
                        "ROLLBACK requires TRANSACTION, TO or to end at this point, got {:?}",
                        self.cur()?.ttype
                    ),
                    self.cur()?,
                    Rule::Syntax,
                );
                err.doc_url = Some("https://www.sqlite.org/lang_transaction.html");
                self.errors.push(err);
            }
        }

        // optional TRANSACTION
        if self.is(Type::Keyword(Keyword::TRANSACTION)) {
            self.advance();
        }

        // optional TO
        if self.is(Type::Keyword(Keyword::TO)) {
            self.advance();

            // optional SAVEPOINT
            if self.is(Type::Keyword(Keyword::SAVEPOINT)) {
                self.advance();
            }

            match self.cur()?.ttype {
                Type::Keyword(Keyword::SAVEPOINT) | Type::Ident(_) | Type::Semicolon => {}
                _ => {
                    let mut err = self.err(
                        "Unexpected Token",
                        &format!(
                            "ROLLBACK requires SAVEPOINT, Ident or to end at this point, got {:?}",
                            self.cur()?.ttype
                        ),
                        self.cur()?,
                        Rule::Syntax,
                    );
                    err.doc_url = Some("https://www.sqlite.org/lang_transaction.html");
                    self.errors.push(err);
                    self.advance();
                }
            }

            if let Type::Ident(str) = &self.cur()?.ttype {
                rollback.save_point = Some(String::from(str));
            } else {
                let mut err = self.err(
                    "Unexpected Token",
                    &format!(
                        "ROLLBACK wants Ident as <savepoint-name>, got {:?}",
                        self.cur()?.ttype
                    ),
                    self.cur()?,
                    Rule::Syntax,
                );
                err.doc_url = Some("https://www.sqlite.org/lang_transaction.html");
                self.errors.push(err);
            }
            self.advance();
        }

        if !self.is(Type::Semicolon) {
            let mut err = self.err(
                "Unexpected Token",
                &format!(
                    "ROLLBACK end as Semicolon expected, got {:?}",
                    self.cur()?.ttype
                ),
                self.cur()?,
                Rule::Syntax,
            );
            err.doc_url = Some("https://www.sqlite.org/lang_transaction.html");
            self.errors.push(err);
            self.advance();
        }

        some_box!(rollback)
    }

    /// https://www.sqlite.org/syntax/commit-stmt.html
    fn commit_stmt(&mut self) -> Option<Box<dyn Node>> {
        let commit: Option<Box<dyn Node>> = some_box!(Commit {
            t: self.cur()?.clone(),
        });

        // skip either COMMIT or END
        self.advance();

        match self.cur()?.ttype {
            // expected end 1
            Type::Semicolon => (),
            // expected end 2, optional
            Type::Keyword(Keyword::TRANSACTION) => self.advance(),
            _ => {
                let mut err = self.err(
                    "Unexpected Token",
                    &format!(
                        "Wanted Keyword(TRANSACTION) or Semicolon, got {:?}",
                        self.cur()?.ttype
                    ),
                    self.cur()?,
                    Rule::Syntax,
                );
                err.doc_url = Some("https://www.sqlite.org/lang_transaction.html");
                self.errors.push(err);
                self.advance();
            }
        }

        if !self.is(Type::Semicolon) {
            self.errors.push(self.err(
                "Unexpected Token",
                &format!(
                    "Wanted no tokens except Semicolon at this point, got {:?}",
                    self.cur()?.ttype
                ),
                self.cur()?,
                Rule::Syntax,
            ));
            self.advance();
        }

        commit
    }

    /// https://www.sqlite.org/syntax/begin-stmt.html
    fn begin_stmt(&mut self) -> Option<Box<dyn Node>> {
        let begin: Begin = Begin {
            t: self.cur()?.clone(),
        };

        // skip BEGIN
        self.advance();

        // skip modifiers
        match self.cur()?.ttype {
            // only BEGIN
            Type::Semicolon => return some_box!(begin),
            Type::Keyword(Keyword::DEFERRED)
            | Type::Keyword(Keyword::IMMEDIATE)
            | Type::Keyword(Keyword::EXCLUSIVE) => self.advance(),
            _ => {}
        }

        match self.cur()?.ttype {
            Type::Semicolon => return some_box!(begin),
            // ending
            Type::Keyword(Keyword::TRANSACTION) => self.advance(),
            Type::Keyword(Keyword::DEFERRED)
            | Type::Keyword(Keyword::IMMEDIATE)
            | Type::Keyword(Keyword::EXCLUSIVE) => {
                let mut err = self.err(
                    "Unexpected Token",
                    "BEGIN does not allow multiple transaction behaviour modifiers",
                    self.cur()?,
                    Rule::Syntax,
                );
                err.doc_url = Some("https://www.sqlite.org/lang_transaction.html");
                self.errors.push(err);
                // TODO: think about if this is smart at this point, skipping to the next ; could
                // be skipping too many tokens
                self.skip_until_semicolon();
            }
            _ => {
                let mut err = self.err(
                    "Unexpected Token",
                    &format!(
                        "Wanted any of TRANSACTION, DEFERRED, IMMEDIATE or EXCLUSIVE before this point, got {:?}",
                        self.cur()?.ttype
                    ),
                    self.cur()?,
                    Rule::Syntax,
                );
                err.doc_url = Some("https://www.sqlite.org/lang_transaction.html");
                self.errors.push(err);
            }
        }

        if !self.is(Type::Semicolon) {
            self.errors.push(self.err(
                "Unexpected Token",
                &format!(
                    "Wanted no tokens except Semicolon at this point, got {:?}",
                    self.cur()?.ttype
                ),
                self.cur()?,
                Rule::Syntax,
            ));
            self.advance();
        }

        some_box!(begin)
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
            return some_box!(v);
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

        some_box!(v)
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
                let s: Option<Box<dyn Node>> = some_box!(Literal { t: cur.clone() });
                // skipping over the current character
                self.advance();
                s
            }
            _ => {
                let mut err = self.err("Unexpected Token", &format!("Wanted a literal (any of number,string,blob,null,true,false,CURRENT_TIME,CURRENT_DATE,CURRENT_DATE), got {:?}", cur.ttype),cur, Rule::Syntax);
                err.doc_url = Some("https://www.sqlite.org/syntax/literal-value.html");
                self.errors.push(err);
                self.advance();
                None
            }
        }
    }
}
