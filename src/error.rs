use std::io::BufRead;

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
}

pub enum Color {
    Red,
    Reset,
    Blue,
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

        if content.len() == 0 {
            return;
        }

        print_str_colored(" => ", Color::Blue);
        print_str!(self.file);
        print_str!('\n');

        let lines = content.lines().map(|x| x.unwrap()).collect::<Vec<_>>();

        // eof should always highlight the last line
        match &self.rule {
            &Rule::NoStatements => {
                self.line = lines.len() - 1;
                self.end = 0;
            }
            _ => (),
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
        print_str_colored("    =", Color::Blue);
        print_str!(" note: ");
        print_str!(self.note);
        print_str!('\n');
    }
}
