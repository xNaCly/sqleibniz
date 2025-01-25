use std::{fmt::Display, string::FromUtf8Error};

/// builder implements a string builder, in its api similar to [strings.Builder](https://pkg.go.dev/strings#Builder)
pub struct Builder {
    buffer: Vec<u8>,
}

impl Builder {
    pub fn new() -> Self {
        Builder { buffer: Vec::new() }
    }

    pub fn with_capacity(cap: usize) -> Self {
        Builder {
            buffer: Vec::with_capacity(cap),
        }
    }

    pub fn write_char(&mut self, char: char) {
        self.buffer.push(char as u8);
    }

    pub fn write_byte(&mut self, byte: u8) {
        self.buffer.push(byte);
    }

    pub fn write_str(&mut self, str: &str) {
        self.buffer.append(&mut str.as_bytes().to_vec());
    }

    pub fn write_string(&mut self, string: String) {
        self.buffer.append(&mut string.into_bytes())
    }

    /// string consumes the Builder
    pub fn string(self) -> String {
        match String::from_utf8(self.buffer) {
            Ok(string) => string,
            Err(_) => String::from("<failed to stringify Builder::buffer"),
        }
    }
}

impl Display for Builder {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = match String::from_utf8(self.buffer.clone()) {
            Ok(string) => string,
            Err(_) => String::from("<failed to stringify Builder::buffer"),
        };
        write!(f, "{}", string)
    }
}
