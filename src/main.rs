#![allow(dead_code)]
use std::{env, fs, process::exit, vec};

use error::{print_str_colored, warn};
use lexer::Lexer;
use rules::{Config, Disabled};

/// error does formatting and highlighting for errors
mod error;
/// lexer converts the input into a stream of token for the parser
mod lexer;
/// rules controls the error display and configuration
mod rules;
/// types holds all shared types between the above modules
mod types;

struct FileResult {
    name: String,
    errors: usize,
    ignored_errors: usize,
}

fn main() {
    if env::args().len() == 1 {
        error::err("no source file(s) provided, exiting");
        exit(1);
    }

    let mut config = Config {
        disabled: Disabled { rules: vec![] },
    };
    if let Ok(config_str) = fs::read_to_string("leibniz.toml") {
        if let Ok(conf) = toml::from_str(&config_str) {
            config = conf
        }
    }

    if config.disabled.rules.is_empty() {
        warn("Ignoring the following diagnostics, according to 'leibniz.toml':");
        for rule in &config.disabled.rules {
            print_str_colored(" -> ", error::Color::Blue);
            println!("{}", rule.name())
        }
    }

    let mut files = env::args()
        .skip(1)
        .map(|name| FileResult {
            name,
            errors: 0,
            ignored_errors: 0,
        })
        .collect::<Vec<FileResult>>();
    for file in &mut files {
        let content = match fs::read(&file.name) {
            Ok(c) => c,
            Err(err) => {
                error::err(&format!("failed to read file '{}': {}", file.name, err));
                exit(1);
            }
        };
        let mut ignored_errors = 0;
        let mut lexer = Lexer::init(&content, file.name.clone());
        let toks = lexer.run();
        dbg!(toks);
        lexer.errors.retain(|e| {
            if config.disabled.rules.contains(&e.rule) {
                ignored_errors += 1;
                false
            } else {
                true
            }
        });
        if !lexer.errors.is_empty() {
            error::print_str_colored(
                &format!("{:=^72}\n", format!(" {} ", file.name)),
                error::Color::Blue,
            );
            let error_count = lexer.errors.len();
            for (i, e) in &mut lexer.errors.iter_mut().enumerate() {
                e.print(&content);
                if i + 1 != error_count {
                    println!()
                }
            }
        }
        // dbg!(toks);
        file.errors = lexer.errors.len();
        file.ignored_errors = ignored_errors;
    }

    error::print_str_colored(&format!("{:=^72}\n", " Summary "), error::Color::Blue);
    for file in &files {
        error::print_str_colored(
            &format!(
                "[{}]",
                match file.errors {
                    0 => '+',
                    _ => '-',
                }
            ),
            match file.errors {
                0 => error::Color::Green,
                _ => error::Color::Red,
            },
        );
        println!(" {}:", file.name);
        match file.errors {
            0 => println!("    {} Error(s) detected", file.errors,),
            _ => error::print_str_colored(
                &format!("    {} Error(s) detected\n", file.errors),
                error::Color::Red,
            ),
        }
        match file.ignored_errors {
            0 => println!("    {} Error(s) ignored", file.ignored_errors),
            _ => error::print_str_colored(
                &format!("    {} Error(s) ignored\n", file.ignored_errors),
                error::Color::Yellow,
            ),
        }
    }
    println!();
    print_str_colored("=>", error::Color::Blue);
    let verified = files.iter().filter(|f| f.errors == 0).count();
    println!(
        " {}/{} Files verified successfully, {} verification failed.",
        verified,
        files.len(),
        files.len() - verified
    )
}
