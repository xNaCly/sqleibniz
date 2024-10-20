use crate::{
    error::Error,
    rules::Rule,
    types::{Keyword, Token, Type},
};
use nodes::{Literal, Node};

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
        if let Some(tt) = self.cur() {
            return tt.ttype == t;
        }
        false
    }

    /// checks if type of current token is equal to t, otherwise pushes an error
    fn consume(&mut self, t: Type) {
        if self.is(t) {
            self.advance();
        }
    }

    pub fn parse(&mut self) -> Vec<Option<Box<dyn Node>>> {
        self.sql_stmt_list()
    }

    /// see: https://www.sqlite.org/syntax/sql-stmt-list.html
    fn sql_stmt_list(&mut self) -> Vec<Option<Box<dyn Node>>> {
        let mut r = vec![];
        while !self.is_eof() {
            r.push(self.sql_stmt());
            self.consume(Type::Semicolon);
        }
        r
    }

    /// see: https://www.sqlite.org/syntax/sql-stmt.html
    fn sql_stmt(&mut self) -> Option<Box<dyn Node>> {
        match self.cur()?.ttype {
            // TODO: all paths HERE
            _ => self.literal_value(),
        }
    }

    /// see: https://www.sqlite.org/syntax/literal-value.html
    fn literal_value(&mut self) -> Option<Box<dyn Node>> {
        let cur = self.cur()?;
        let r: Option<Box<dyn Node>> = match cur.ttype {
            Type::String(_)
            | Type::Number(_)
            | Type::Blob(_)
            | Type::Keyword(Keyword::NULL)
            | Type::Boolean(_)
            | Type::Keyword(Keyword::CURRENT_TIME)
            | Type::Keyword(Keyword::CURRENT_DATE)
            | Type::Keyword(Keyword::CURRENT_TIMESTAMP) => {
                Some(Box::new(Literal { t: cur.clone() }))
            }
            _ => {
                let mut err = self.err("Unexpected Token", &format!("Wanted a literal (any of number,string,blob,null,true,false,CURRENT_TIME,CURRENT_DATE,CURRENT_DATE), got {:?}", cur.ttype),cur, Rule::Syntax);
                err.doc_url = Some("https://www.sqlite.org/syntax/literal-value.html");
                self.errors.push(err);
                None
            }
        };
        self.advance();
        r
    }
}
