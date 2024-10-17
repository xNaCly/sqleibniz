use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq)]
pub enum Rule {
    /// Source file is empty
    NoContent,
    /// Source file is not empty but holds no statements
    NoStatements,
    /// Source file contains constructs sqleibniz does not yet understand
    Unimplemented,
    /// Source file contains an unterminated string
    UnterminatedString,
    /// The source file contains an unknown character
    UnknownCharacter,
    /// The source file contains an invalid numeric literal
    InvalidNumericLiteral,
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
            Self::UnknownCharacter => "UnknownCharacter",
            Self::InvalidNumericLiteral => "InvalidNumericLiteral",
        }
    }

    pub fn description(&self) -> &str {
        match self {
            Self::NoContent => "Source file is empty",
            Self::NoStatements => "Source file is not empty but holds no statements",
            Self::Unimplemented => {
                "Source file contains constructs sqleibniz does not yet understand"
            }
            Self::UnterminatedString => "Source file contains an unterminated string",
            Self::UnknownCharacter => "The source file contains an unknown character",
            Self::InvalidNumericLiteral => "The source file contains an invalid numeric literal",
        }
    }
}
