use serde::Deserialize;

#[derive(Deserialize, Debug, PartialEq, Clone)]
/// Rule is attached to each error and can be supplied to sqleibniz via the Config structure serialized in ./leibniz.toml
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
    /// The source file contains an invalid numeric literal, either overflow or incorrect syntax
    InvalidNumericLiteral,
    /// The source file contains an invalid blob literal, either bad hex data (a-f,A-F,0-9) or
    /// incorrect syntax
    InvalidBlob,
    /// The source file contains a structure with incorrect syntax
    Syntax,
    /// The source file is missing a semicolon
    Semicolon,
}

#[derive(Deserialize, Debug)]
/// Allows for disabling diagnostics and turning off behaviours
pub struct Disabled {
    pub rules: Vec<Rule>,
}

#[derive(Deserialize, Debug)]
/// Configuration is expected to be at ./leibniz.toml - its existence is not necessary for the program invocation
pub struct Config {
    /// holds the rules that the user wants to not see errors for.
    pub disabled: Disabled,
}

impl Rule {
    pub fn name(&self) -> &str {
        match self {
            Self::NoContent => "NoContent",
            Self::NoStatements => "NoStatements",
            Self::Unimplemented => "Unimplemented",
            Self::UnterminatedString => "UnterminatedString",
            Self::UnknownCharacter => "UnknownCharacter",
            Self::InvalidNumericLiteral => "InvalidNumericLiteral",
            Self::InvalidBlob => "InvalidBlob",
            Self::Syntax => "Syntax",
            Self::Semicolon => "Semicolon",
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
            Self::InvalidBlob => "The source file contains an invalid blob literal",
            Self::Syntax => "The source file contains a structure with incorrect syntax",
            Self::Semicolon => "The source file is missing a semicolon",
        }
    }
}
