#[derive(Debug)]
pub enum Keyword {
    SELECT,
    FROM,
    WHERE,
}

impl Keyword {
    pub fn from_str(s: &str) -> Option<Keyword> {
        Some(match s.to_lowercase().as_str() {
            "select" => Keyword::SELECT,
            "from" => Keyword::FROM,
            _ => return None,
        })
    }
}

#[derive(Debug)]
pub enum Type {
    Keyword(Keyword),
}

#[derive(Debug)]
pub struct Token {
    pub ttype: Type,
    pub pos: usize,
}
