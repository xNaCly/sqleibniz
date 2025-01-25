use crate::types::Token;

pub mod builder;

/// highlight_string performs syntax highlighting on the given [line], depending on the tokens in
/// [token_on_line]. The generated output is writen to the [builder::Builder], thats passed into
/// the function
pub fn highlight_string(builder: &mut builder::Builder, token_on_line: &[Token], line: &str) {
    builder.write_str(line);
}
