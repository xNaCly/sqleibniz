#![allow(dead_code)]
use std::{env, fs};

use lexer::Lexer;

mod error;
mod lexer;
mod types;

fn main() {
    let file_path = env::args().nth(1).expect("No source file provided.");
    let content = fs::read(file_path.clone()).expect("Failed to read file");
    let mut lexer = Lexer::init(&content, file_path);
    let toks = lexer.run();
    if lexer.errors.len() != 0 {
        for e in &lexer.errors {
            e.print(&content);
            println!();
        }
    }
    if lexer.errors.len() != 0 {
        error::print_str_colored("error", error::Color::Red);
        println!(": exiting due to {} error{}", lexer.errors.len(), {
            if lexer.errors.len() > 1 {
                "s"
            } else {
                ""
            }
        });
        return;
    }
    println!("{:?}", toks);
}
