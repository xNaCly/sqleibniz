mod keyword;

/// this shit is really fucking idiotic, but i have to reexport
/// private identifiers, rust you are fucking weird
pub use self::keyword::Keyword;

#[derive(Debug)]
pub enum Type {
    Keyword(keyword::Keyword),
    Ident(String),
    Number(f64),
    String(String),
    Blob(Vec<u8>),
    Boolean(bool),
    Dot,
    Asteriks,
    Semicolon,
    Percent,
    Comma,
}

#[derive(Debug)]
pub struct Token {
    pub ttype: Type,
    pub start: usize,
    pub end: usize,
}
