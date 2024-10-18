use std::{fs, io::BufRead, path::PathBuf};

use crate::rules::Rule;

#[derive(Debug)]
pub struct Error {
    pub file: String,
    pub line: usize,
    pub rule: Rule,
    pub note: String,
    pub msg: String,
    pub start: usize,
    pub end: usize,
    pub doc_url: Option<&'static str>,
}

pub enum Color {
    Red,
    Reset,
    Blue,
    Cyan,
    Green,
    Yellow,
}

impl Color {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Reset => "\x1b[0m",
            Self::Red => "\x1b[31m",
            Self::Blue => "\x1b[94m",
            Self::Green => "\x1b[92m",
            Self::Yellow => "\x1b[93m",
            Self::Cyan => "\x1b[96m",
        }
    }
}

pub fn warn(s: &str) {
    print_str_colored("warn", Color::Yellow);
    println!(": {}", s);
}

pub fn err(s: &str) {
    print_str_colored("error", Color::Red);
    println!(": {}", s);
}

pub fn print_str_colored(s: &str, c: Color) {
    print!("{}{}{}", c.as_str(), s, Color::Reset.as_str());
}

macro_rules! print_str {
    ($s:expr) => {
        print!("{}", $s);
    };
}

impl Error {
    pub fn print(&mut self, content: &Vec<u8>) {
        print_str_colored("error", Color::Red);
        print_str!("[");
        print_str_colored(self.rule.to_str(), Color::Red);
        print_str!("]: ");
        print_str!(&self.msg);
        println!();

        if content.is_empty() {
            return;
        }

        print_str_colored(" -> ", Color::Blue);
        // the file is not absolut, this resolves symlinks and stuff
        let file_path = match fs::canonicalize(PathBuf::from(&self.file)) {
            Ok(path) => path.into_os_string().into_string().unwrap_or_default(),
            _ => self.file.clone(),
        };
        print_str_colored(&file_path, Color::Cyan);
        // zero based indexing, we need human friendly numbers here
        print_str_colored(
            &format!(":{}:{}", self.line + 1, self.start + 1),
            Color::Yellow,
        );
        println!();

        let lines = content.lines().map(|x| x.unwrap()).collect::<Vec<_>>();

        // eof should always highlight the last line
        if let &Rule::NoStatements = &self.rule {
            self.line = lines.len() - 1;
            self.end = 0;
        }

        if self.line >= 2 {
            if let Some(first_line) = lines.get(self.line - 2) {
                print_str_colored(&format!(" {:02} | ", self.line - 1), Color::Blue);
                print_str!(first_line);
                println!()
            }

            if let Some(sec_line) = lines.get(self.line - 1) {
                print_str_colored(&format!(" {:02} | ", self.line), Color::Blue);
                print_str!(sec_line);
                println!()
            }
        }

        let offending_line = lines.get(self.line).unwrap();
        print_str_colored(&format!(" {:02} | ", self.line + 1), Color::Blue);
        print_str!(offending_line);
        print_str_colored("\n    |", Color::Blue);

        let mut repeat = 1;
        if self.end > self.start {
            repeat = self.end - self.start;
        }

        print_str_colored(
            &format!(
                " {}{} error occurs here.\n",
                " ".repeat(self.start),
                "^".repeat(repeat)
            ),
            Color::Red,
        );

        if let Some(first_line) = lines.get(self.line + 1) {
            print_str_colored(&format!(" {:02} | ", self.line + 2), Color::Blue);
            print_str!(first_line);
            println!()
        }

        if let Some(sec_line) = lines.get(self.line + 2) {
            print_str_colored(&format!(" {:02} | ", self.line + 3), Color::Blue);
            print_str!(sec_line);
            println!()
        }

        print_str_colored("    |\n", Color::Blue);
        print_str_colored("    ~ note: ", Color::Blue);
        print_str!(self.note);
        println!();

        print_str_colored("  * ", Color::Blue);
        print_str_colored(self.rule.to_str(), Color::Blue);
        print_str!(": ");
        print_str!(self.rule.description());
        println!();

        if self.doc_url.is_some() {
            println!();
            print_str_colored(" docs", Color::Blue);
            print_str!(": ");
            print_str!(self.doc_url.unwrap());
            println!();
        }
    }
}
