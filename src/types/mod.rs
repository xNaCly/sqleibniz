mod keyword;
pub use self::keyword::Keyword; // this shit is really fucking idiotic, but i have to reexport
                                // private identifiers, rust you are fucking weird

#[derive(Debug)]
pub enum Type {
    Keyword(keyword::Keyword),
    Ident(String),
    Number(f64),
    String(String),
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
