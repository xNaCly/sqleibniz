#[cfg(feature = "trace")]
use tracer::Tracer;

use crate::{
    error::{Error, ImprovedLine},
    rules::Rule,
    types::{Keyword, Token, Type},
};

mod nodes;
mod tests;
mod tracer;

/// prints a parser function call trace if #[cfg(feature = "trace")]
macro_rules! trace {
    ($tracer:expr, $fn:literal, $tok:expr) => {
        #[cfg(feature = "trace")]
        $tracer.call($fn, $tok.map(|t| t.ttype.clone()));
    };
}

/// restores trace indent if #[cfg(feature = "trace")]
macro_rules! detrace {
    ($tracer:expr) => {
        #[cfg(feature = "trace")]
        {
            $tracer.indent -= 1;
        }
    };
}

pub struct Parser<'a> {
    pos: usize,
    tokens: Vec<Token>,
    name: &'a str,
    pub errors: Vec<Error>,
    #[cfg(feature = "trace")]
    tracer: tracer::Tracer,
}

/// wrap argument in Some(Box::new(_))
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
            #[cfg(feature = "trace")]
            tracer: Tracer::new(),
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

    fn skip_until_semicolon_or_eof(&mut self) {
        while !self.is_eof() && !self.is(Type::Semicolon) {
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

    /// checks if current token is not semicolon, if it isnt pushes an error
    fn expect_end(&mut self, doc: &'static str) -> Option<()> {
        if !self.is(Type::Semicolon) {
            let mut err = self.err(
                "Unexpected Statement Continuation",
                &format!(
                    "End of statement via Semicolon expected, got {:?}",
                    self.cur()?.ttype
                ),
                self.cur()?,
                Rule::Syntax,
            );
            if !doc.is_empty() {
                err.doc_url = Some(doc);
            }
            self.errors.push(err);
            self.advance();
        }
        None
    }

    fn consume_ident(
        &mut self,
        doc: &'static str,
        expected_ident_name: &'static str,
    ) -> Option<String> {
        if let Type::Ident(ident) = &self.cur()?.ttype {
            let i = ident.clone();
            self.advance();
            Some(i)
        } else {
            let mut err = self.err(
                "Unexpected Token",
                &format!(
                    "Expected Ident(<{}>), got {:?}",
                    expected_ident_name,
                    self.cur()?.ttype
                ),
                self.cur()?,
                Rule::Syntax,
            );
            err.doc_url = Some(doc);
            self.errors.push(err);
            self.skip_until_semicolon_or_eof();
            None
        }
    }

    pub fn parse(&mut self) -> Vec<Option<Box<dyn nodes::Node>>> {
        trace!(self.tracer, "parse", self.cur());
        let r = self.sql_stmt_list();
        detrace!(self.tracer);
        r
    }

    /// see: https://www.sqlite.org/syntax/sql-stmt-list.html
    fn sql_stmt_list(&mut self) -> Vec<Option<Box<dyn nodes::Node>>> {
        trace!(self.tracer, "sql_stmt_list", self.cur());
        let mut r = vec![];
        while !self.is_eof() {
            if let Some(Token {
                ttype: Type::InstructionExpect,
                ..
            }) = self.cur()
            {
                // skip all token until the statement ends
                self.skip_until_semicolon_or_eof();
                // skip ';'
                self.consume(Type::Semicolon);
                continue;
            }
            let stmt = self.sql_stmt_prefix();
            if stmt.is_some() {
                r.push(stmt);
            }
            self.consume(Type::Semicolon);
        }
        detrace!(self.tracer);
        r
    }

    fn sql_stmt_prefix(&mut self) -> Option<Box<dyn nodes::Node>> {
        trace!(self.tracer, "sql_stmt_prefix", self.cur());
        let r: Option<Box<dyn nodes::Node>> = match self.cur()?.ttype {
            Type::Keyword(Keyword::EXPLAIN) => {
                let mut e = nodes::Explain {
                    t: self.cur()?.clone(),
                    children: None,
                };
                // skip EXPLAIN
                self.advance();

                // path for EXPLAIN->QUERY->PLAN
                if self.is(Type::Keyword(Keyword::QUERY)) {
                    self.advance();
                    self.consume(Type::Keyword(Keyword::PLAN));
                }

                // else path is EXPLAIN->*_stmt
                e.children = self.sql_stmt().map(|x| vec![x]);
                some_box!(e)
            }
            _ => self.sql_stmt(),
        };
        detrace!(self.tracer);
        r
    }

    /// see: https://www.sqlite.org/syntax/sql-stmt.html
    fn sql_stmt(&mut self) -> Option<Box<dyn nodes::Node>> {
        trace!(self.tracer, "sql_stmt", self.cur());
        let r = match self.cur()?.ttype {
            // TODO: add new statement starts here
            // Type::Keyword(Keyword::ATTACH) => self.attach_stmt(),
            Type::Keyword(Keyword::REINDEX) => self.reindex_stmt(),
            Type::Keyword(Keyword::RELEASE) => self.release_stmt(),
            Type::Keyword(Keyword::SAVEPOINT) => self.savepoint_stmt(),
            Type::Keyword(Keyword::DROP) => self.drop_stmt(),
            Type::Keyword(Keyword::ANALYZE) => self.analyse_stmt(),
            Type::Keyword(Keyword::DETACH) => self.detach_stmt(),
            Type::Keyword(Keyword::ROLLBACK) => self.rollback_stmt(),
            Type::Keyword(Keyword::COMMIT) | Type::Keyword(Keyword::END) => self.commit_stmt(),
            Type::Keyword(Keyword::BEGIN) => self.begin_stmt(),
            Type::Keyword(Keyword::VACUUM) => self.vacuum_stmt(),

            // statement should not start with a semicolon 󰚌
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
            Type::Ident(_) => {
                if let Type::Ident(name) = &self.cur()?.ttype {
                    self.errors.push(self.err(
                        "Unknown Keyword",
                        &format!(
                            "'{}' is not a know keyword, did you mean: \n\t- {}",
                            name,
                            Keyword::suggestions(name).join("\n\t- ").as_str()
                        ),
                        self.cur()?,
                        Rule::UnknownKeyword,
                    ));
                }
                self.skip_until_semicolon_or_eof();
                None
            }
            Type::Keyword(_) => {
                self.errors.push(self.err(
                    "Unimplemented",
                    &format!(
                        "sqleibniz can not yet analyse the token {:?}, skipping ahead to next statement",
                        self.cur()?.ttype
                    ),
                    self.cur()?,
                    Rule::Unimplemented,
                ));
                self.skip_until_semicolon_or_eof();
                None
            }
            _ => {
                self.errors.push(self.err(
                    "Unknown Token",
                    &format!(
                        "sqleibniz does not understand the token {:?}, skipping ahead to next statement",
                        self.cur()?.ttype
                    ),
                    self.cur()?,
                    Rule::Unimplemented,
                ));
                self.skip_until_semicolon_or_eof();
                None
            }
        };

        detrace!(self.tracer);
        r
    }

    // TODO: add new statement function here *_stmt()
    // fn $1_stmt(&mut self) -> Option<Box<dyn nodes::Node>> {}

    /// https://www.sqlite.org/syntax/reindex-stmt.html
    fn reindex_stmt(&mut self) -> Option<Box<dyn nodes::Node>> {
        trace!(self.tracer, "reindex_stmt", self.cur());
        let mut r = nodes::Reindex {
            t: self.cur()?.clone(),
            collation_or_schema: None,
            children: None,
        };
        self.advance();

        // REINDEX has a path with no further nodes
        if self.is(Type::Semicolon) {
            return some_box!(r);
        }

        // either collation_name, schema_name of schema_name.table_or_index_name or table_or_index_name
        let mut collation_or_schema_or_table_or_view = self.consume_ident(
            "https://www.sqlite.org/syntax/reindex-stmt.html",
            "collation_or_schema_or_table_or_index",
        )?;

        // branch for schema_name.table_or_index_name
        if self.is(Type::Dot) {
            collation_or_schema_or_table_or_view.push('.');
            self.advance();
            collation_or_schema_or_table_or_view.push_str(&self.consume_ident(
                "https://www.sqlite.org/syntax/reindex-stmt.html",
                "table_or_index_name",
            )?);
        }

        r.collation_or_schema = Some(collation_or_schema_or_table_or_view);

        self.expect_end("https://www.sqlite.org/syntax/reindex-stmt.html");

        detrace!(self.tracer);
        some_box!(r)
    }

    /// https://www.sqlite.org/syntax/attach-stmt.html
    fn attach_stmt(&mut self) -> Option<Box<dyn nodes::Node>> {
        trace!(self.tracer, "attach_stmt", self.cur());
        let mut a = nodes::Attach {
            t: self.cur()?.clone(),
            schema_name: String::new(),
            children: None,
        };
        self.advance();

        if self.is(Type::Keyword(Keyword::DATABASE)) {
            self.advance();
        }

        // TODO: expr here

        self.consume(Type::Keyword(Keyword::AS));

        a.schema_name =
            self.consume_ident("https://www.sqlite.org/lang_attach.html", "schema_name")?;

        self.expect_end("https://www.sqlite.org/lang_attach.html");
        detrace!(self.tracer);
        some_box!(a)
    }

    /// https://www.sqlite.org/syntax/release-stmt.html
    fn release_stmt(&mut self) -> Option<Box<dyn nodes::Node>> {
        trace!(self.tracer, "release_stmt", self.cur());
        let mut r = nodes::Release {
            t: self.cur()?.clone(),
            savepoint_name: String::new(),
            children: None,
        };
        self.advance();

        if self.is(Type::Keyword(Keyword::SAVEPOINT)) {
            self.advance();
        }

        r.savepoint_name = self.consume_ident(
            "https://www.sqlite.org/syntax/release-stmt.html",
            "savepoint_name",
        )?;

        self.expect_end("https://www.sqlite.org/syntax/release-stmt.html");
        detrace!(self.tracer);
        some_box!(r)
    }

    /// https://www.sqlite.org/syntax/savepoint-stmt.html
    fn savepoint_stmt(&mut self) -> Option<Box<dyn nodes::Node>> {
        trace!(self.tracer, "savepoint_stmt ", self.cur());
        let mut s = nodes::Savepoint {
            t: self.cur()?.clone(),
            savepoint_name: String::new(),
            children: None,
        };
        self.advance();
        s.savepoint_name = self.consume_ident(
            "https://www.sqlite.org/syntax/savepoint-stmt.html",
            "savepoint_name",
        )?;
        self.expect_end("https://www.sqlite.org/lang_savepoint.html");

        detrace!(self.tracer);
        some_box!(s)
    }

    /// https://www.sqlite.org/lang_dropindex.html
    /// https://www.sqlite.org/lang_droptable.html
    /// https://www.sqlite.org/lang_droptrigger.html
    /// https://www.sqlite.org/lang_dropview.html
    fn drop_stmt(&mut self) -> Option<Box<dyn nodes::Node>> {
        trace!(self.tracer, "drop_stmt ", self.cur());
        let mut drop = nodes::Drop {
            t: self.cur()?.clone(),
            if_exists: false,
            // dummy value
            ttype: Keyword::NULL,
            argument: String::new(),
            children: None,
        };
        self.advance();

        match self.cur()?.ttype {
            Type::Keyword(Keyword::INDEX) => (),
            Type::Keyword(Keyword::TABLE) => (),
            Type::Keyword(Keyword::TRIGGER) => (),
            Type::Keyword(Keyword::VIEW) => (),
            _ => {
                let mut err = self.err(
                        "Unexpected Token",
                        &format!(
                            "DROP requires either TRIGGER, TABLE, TRIGGER or VIEW at this point, got {:?}",
                            self.cur()?.ttype
                        ),
                        self.cur()?,
                        Rule::Syntax,
                    );
                err.doc_url = Some("https://www.sqlite.org/lang.html");
                self.errors.push(err);
                self.advance();
                return None;
            }
        }

        // we checked if the keyword is valid above
        if let Type::Keyword(keyword) = &self.cur()?.ttype {
            drop.ttype = keyword.clone();
        }

        // skip either INDEX;TABLE;TRIGGER or VIEW
        self.advance();

        if self.is(Type::Keyword(Keyword::IF)) {
            self.advance();
            self.consume(Type::Keyword(Keyword::EXISTS));
            drop.if_exists = true;
        }

        if let Type::Ident(schema_name) = self.cur()?.ttype.clone() {
            // table/index/view/trigger of a schema_name
            drop.argument.push_str(&schema_name);
            if self.next_is(Type::Dot) {
                // skip Type::Ident from above
                self.advance();
                // skip Type::Dot
                self.advance();
                if let Type::Ident(index_trigger_table_view) = self.cur()?.ttype.clone() {
                    drop.argument.push('.');
                    drop.argument.push_str(&index_trigger_table_view);
                } else {
                    let mut err = self.err(
                        "Unexpected Token",
                        &format!(
                            "DROP requires Ident(<index_or_trigger_or_table_or_view>) after Dot and Ident(<schema_name>), got {:?}",
                            self.cur()?.ttype
                        ),
                        self.cur()?,
                        Rule::Syntax,
                    );
                    err.doc_url = Some("https://www.sqlite.org/lang_dropview.html https://www.sqlite.org/lang_droptrigger.html https://www.sqlite.org/lang_droptable.html https://www.sqlite.org/lang_dropindex.html");
                    self.advance();
                    self.errors.push(err);
                }
            }
            self.advance();
        } else {
            let mut err = self.err(
                        "Unexpected Token",
                        &format!(
                            "DROP requires Ident(<index_or_trigger_or_table_or_view>) or Ident(<schema_name>).Ident(<index_or_trigger_or_table_or_view>), got {:?}",
                            self.cur()?.ttype
                        ),
                        self.cur()?,
                        Rule::Syntax,
                    );
            err.doc_url = Some("https://www.sqlite.org/lang_dropview.html https://www.sqlite.org/lang_droptrigger.html https://www.sqlite.org/lang_droptable.html https://www.sqlite.org/lang_dropindex.html");
            self.errors.push(err);
            return None;
        }

        detrace!(self.tracer);
        some_box!(drop)
    }

    /// https://www.sqlite.org/syntax/analyze-stmt.html
    fn analyse_stmt(&mut self) -> Option<Box<dyn nodes::Node>> {
        trace!(self.tracer, "analyse_stmt", self.cur());
        let mut a = nodes::Analyze {
            t: self.cur()?.clone(),
            schema_index_or_table_name: None,
            schema_with_table_or_index_name: None,
            children: None,
        };

        self.advance();

        match &self.cur()?.ttype {
            // ANALYZE schema_name.table_or_index_name
            Type::Ident(ident) if self.next_is(Type::Dot) => {
                let mut schema_name = String::from(ident);
                // skip ident
                self.advance();
                // skip dot
                self.advance();

                if let Type::Ident(ident) = &self.cur()?.ttype {
                    schema_name.push('.');
                    schema_name += ident.as_str();
                    a.schema_with_table_or_index_name = Some(schema_name);
                    self.advance();
                } else {
                    let mut err = self.err(
                        "Unexpected Token",
                        &format!(
                            "ANALYZE requires Ident(<table_or_index_name>) after Dot and Ident(<schema_name>), got {:?}",
                            self.cur()?.ttype
                        ),
                        self.cur()?,
                        Rule::Syntax,
                    );
                    err.doc_url = Some("https://www.sqlite.org/lang_analyze.html");
                    self.advance();
                    self.errors.push(err);
                }
            }
            // ANALYZE schema_name
            // ANALYZE index_or_table_name
            Type::Ident(ident) => {
                a.schema_index_or_table_name = Some(ident.into());
                self.advance();
            }
            _ => (),
        }

        self.expect_end("https://www.sqlite.org/lang_analyze.html");
        detrace!(self.tracer);
        some_box!(a)
    }

    /// https://www.sqlite.org/syntax/detach-stmt.html
    fn detach_stmt(&mut self) -> Option<Box<dyn nodes::Node>> {
        trace!(self.tracer, "detach_stmt", self.cur());
        let t = self.cur()?.clone();
        self.advance();

        // skip optional DATABASE path
        if self.is(Type::Keyword(Keyword::DATABASE)) {
            self.advance();
        }

        let r: Option<Box<dyn nodes::Node>> =
            if let Type::Ident(schema_name) = self.cur()?.ttype.clone() {
                self.advance();
                some_box!(nodes::Detach {
                    t,
                    schema_name: schema_name.into(),
                    children: None,
                })
            } else {
                let mut err = self.err(
                    "Unexpected Token",
                    &format!(
                        "DETACH requires Ident(<schema_name>) at this point, got {:?}",
                        self.cur()?.ttype
                    ),
                    self.cur()?,
                    Rule::Syntax,
                );
                err.doc_url = Some("https://www.sqlite.org/lang_detach.html");
                self.errors.push(err);
                None
            };
        self.expect_end("https://www.sqlite.org/lang_detach.html");
        detrace!(self.tracer);
        r
    }

    /// https://www.sqlite.org/syntax/rollback-stmt.html
    fn rollback_stmt(&mut self) -> Option<Box<dyn nodes::Node>> {
        trace!(self.tracer, "rollback_stmt", self.cur());
        let mut rollback = nodes::Rollback {
            t: self.cur()?.clone(),
            save_point: None,
            children: None,
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

        self.expect_end("https://www.sqlite.org/lang_transaction.html");
        detrace!(self.tracer);
        some_box!(rollback)
    }

    /// https://www.sqlite.org/syntax/commit-stmt.html
    fn commit_stmt(&mut self) -> Option<Box<dyn nodes::Node>> {
        trace!(self.tracer, "commit_stmt", self.cur());
        let commit: Option<Box<dyn nodes::Node>> = some_box!(nodes::Commit {
            t: self.cur()?.clone(),
            children: None,
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

        self.expect_end("https://www.sqlite.org/lang_transaction.html");

        detrace!(self.tracer);
        commit
    }

    /// https://www.sqlite.org/syntax/begin-stmt.html
    fn begin_stmt(&mut self) -> Option<Box<dyn nodes::Node>> {
        trace!(self.tracer, "begin_stmt", self.cur());
        let begin: nodes::Begin = nodes::Begin {
            t: self.cur()?.clone(),
            children: None,
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
                self.skip_until_semicolon_or_eof();
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

        self.expect_end("https://www.sqlite.org/lang_transaction.html");

        detrace!(self.tracer);
        some_box!(begin)
    }

    /// https://www.sqlite.org/lang_vacuum.html
    fn vacuum_stmt(&mut self) -> Option<Box<dyn nodes::Node>> {
        trace!(self.tracer, "vacuum_stmt", self.cur());
        let mut v = nodes::Vacuum {
            t: self.cur()?.clone(),
            schema_name: None,
            filename: None,
            children: None,
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

        self.expect_end("https://www.sqlite.org/lang_vacuum.html");

        detrace!(self.tracer);

        some_box!(v)
    }

    /// see: https://www.sqlite.org/syntax/literal-value.html
    fn literal_value(&mut self) -> Option<Box<dyn nodes::Node>> {
        trace!(self.tracer, "literal_value", self.cur());
        detrace!(self.tracer);
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
                let s: Option<Box<dyn nodes::Node>> = some_box!(nodes::Literal {
                    t: cur.clone(),
                    children: None,
                });
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
