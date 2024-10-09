#![allow(dead_code)]
use std::{env, fs, process::exit};

use lexer::Lexer;

mod error;
mod lexer;
mod types;

fn main() {
    if env::args().len() == 1 {
        error::print_err_str("no source file(s) provided, exiting");
        exit(1);
    }
    for file_name in env::args().skip(1).collect::<Vec<String>>() {
        error::print_str_colored(
            &format!("attempting to process '{file_name}\n"),
            error::Color::Blue,
        );
        let content = match fs::read(&file_name) {
            Ok(c) => c,
            Err(err) => {
                error::print_err_str(&format!("failed to read file '{file_name}': {err}"));
                exit(1);
            }
        };
        let mut lexer = Lexer::init(&content, file_name.clone());
        let toks = lexer.run();
        if lexer.errors.len() != 0 {
            for e in &mut lexer.errors {
                e.print(&content);
            }
        }
        error::print_err_str(&format!(
            "failed to verify '{file_name}' due to {} error{}",
            lexer.errors.len(),
            {
                if lexer.errors.len() > 1 {
                    "s"
                } else {
                    ""
                }
            }
        ));
        if lexer.errors.len() == 0 {
            println!("{:?}", toks);
        }

        // final thing, should only be called if no errors in lexer or parser
        if lexer.errors.len() == 0 {
            error::print_str_colored(
                &format!("verified '{file_name}', all good"),
                error::Color::Green,
            );
        }
    }
}
