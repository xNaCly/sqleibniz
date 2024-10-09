#![allow(dead_code)]
use std::{env, fs, process::exit};

use lexer::Lexer;

/// error does formatting and highlighting for errors
mod error;
mod lexer;
mod rules;
mod types;

fn main() {
    if env::args().len() == 1 {
        error::err("no source file(s) provided, exiting");
        exit(1);
    }
    let mut had_errors = false;
    for file_name in env::args().skip(1).collect::<Vec<String>>() {
        error::print_str_colored(
            &format!("attempting to process '{file_name}\n"),
            error::Color::Blue,
        );
        let content = match fs::read(&file_name) {
            Ok(c) => c,
            Err(err) => {
                error::err(&format!("failed to read file '{file_name}': {err}"));
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
        error::err(&format!(
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

        // final thing, should only be called if no errors in lexer or parser
        if lexer.errors.len() == 0 {
            dbg!(toks);
            error::print_str_colored(
                &format!("verified '{file_name}', all good"),
                error::Color::Green,
            );
        } else {
            had_errors = true
        }
    }
    if had_errors {
        exit(1);
    }
}
