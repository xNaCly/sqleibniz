#[derive(Debug)]
pub enum Rule {
    /// NoContent is emitted when the source file is empty
    NoContent,
    /// NoStatements is emitted when the source file is not empty but holds no statements
    NoStatements,
    /// Unimplemented is emitted when the source file contains constructs sqleibniz does not yet
    /// understand
    Unimplemented,
    /// Unterminated is emitted when the source file contains unterminated strings
    UnterminatedString,
}

// TODO: serialize this via serde, convert strings to enum fields, implement the following of these
// rules

pub struct Config {
    pub disabled: Vec<String>,
}

impl Rule {
    pub fn from(s: &str) -> Option<Rule> {
        return None;
    }

    pub fn to_str(&self) -> &str {
        match self {
            Self::NoContent => "NoContent",
            Self::NoStatements => "NoStatements",
            Self::Unimplemented => "Unimplemented",
            Self::UnterminatedString => "UnterminatedString",
        }
    }
}
