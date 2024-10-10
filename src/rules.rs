use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub enum Rule {
    /// NoContent is emitted when the source file is empty
    NoContent,
    /// NoStatements is emitted when the source file is not empty but holds no statements
    NoStatements,
    /// Unimplemented is emitted when the source file contains constructs sqleibniz does not yet understand
    Unimplemented,
    /// Unterminated is emitted when the source file contains unterminated strings
    UnterminatedString,
}

#[derive(Deserialize, Debug)]
/// Allows for disabling diagnostics and turning off behaviours
pub struct Disabled {
    pub rules: Vec<Rule>,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    pub disabled: Disabled,
}

impl Rule {
    pub fn to_str(&self) -> &str {
        match self {
            Self::NoContent => "NoContent",
            Self::NoStatements => "NoStatements",
            Self::Unimplemented => "Unimplemented",
            Self::UnterminatedString => "UnterminatedString",
        }
    }
}
