use lsp_server::ProtocolError;

#[derive(Debug)]
pub struct LspError {
    cause: String,
}

impl std::fmt::Display for LspError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "LspError: {}", self.cause)
    }
}

impl std::error::Error for LspError {}

impl From<&str> for LspError {
    fn from(value: &str) -> Self {
        LspError {
            cause: value.to_string(),
        }
    }
}

impl From<String> for LspError {
    fn from(value: String) -> Self {
        LspError { cause: value }
    }
}

impl From<ProtocolError> for LspError {
    fn from(value: ProtocolError) -> Self {
        Self {
            // man why the fuck is value.0 not exported, i just want to use my error wrapper :(
            cause: value.to_string(),
        }
    }
}
