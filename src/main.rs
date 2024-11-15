#![allow(dead_code)]
use std::{fs, process::exit, vec};

use clap::Parser;
use error::{print_str_colored, warn};
use lexer::Lexer;
use rules::{Config, Disabled, Rule};

/// error does formatting and highlighting for errors
mod error;
/// lexer converts the input into a stream of token for the parser
mod lexer;
/// lsp implements the language server protocol to provide diagnostics, suggestions and snippets for sql based on the sqleibniz tooling
mod lsp;
/// parser converts the token stream into an abstract syntax tree
mod parser;
/// rules controls the error display and configuration
mod rules;
/// types holds all shared types between the above modules
mod types;

/// LSP and analysis cli for sql. Check for valid syntax, semantics and perform dynamic analysis.
#[derive(clap::Parser)]
#[command(about, long_about=None)]
struct Cli {
    /// instruct sqleibniz to ignore the configuration, if found
    #[arg(short, long)]
    ignore_config: bool,

    /// files to analyse
    paths: Vec<String>,

    /// path to the configuration
    #[arg(short = 'c', long, default_value = "leibniz.toml")]
    config: String,

    /// disable stdout/stderr output
    #[arg(short = 's', long)]
    silent: bool,

    /// disable diagnostics by their rules, all are enabled by default - this may change in the
    /// future
    #[arg(short = 'D')]
    #[clap(value_enum)]
    disable: Option<Vec<Rule>>,
}

struct FileResult {
    name: String,
    errors: usize,
    ignored_errors: usize,
}

fn main() {
    let args = Cli::parse();
    if args.paths.is_empty() {
        if !args.silent {
            error::err("no source file(s) provided, exiting");
        }
        exit(1);
    }

    let mut config = Config {
        disabled: Disabled { rules: vec![] },
    };

    if !args.ignore_config {
        match fs::read_to_string(&args.config) {
            Ok(config_str) => match toml::from_str(&config_str) {
                Ok(conf) => config = conf,
                Err(err) => warn(&format!(
                    "Failed to parse configuration file '{}': {}",
                    args.config, err
                )),
            },
            Err(err) => warn(&format!(
                "Failed to read configuration file '{}': {}",
                args.config, err
            )),
        }
    }

    if let Some(rules) = args.disable {
        let mut p = rules.clone();
        config.disabled.rules.append(&mut p);
    }

    if !config.disabled.rules.is_empty() && !args.silent {
        warn("Ignoring the following diagnostics, as specified:");
        for rule in &config.disabled.rules {
            print_str_colored(" -> ", error::Color::Blue);
            println!("{}", rule.name())
        }
    }

    let mut files = args
        .paths
        .into_iter()
        .map(|name| FileResult {
            name,
            errors: 0,
            ignored_errors: 0,
        })
        .collect::<Vec<FileResult>>();

    for file in &mut files {
        let mut errors = vec![];
        let content = match fs::read(&file.name) {
            Ok(c) => c,
            Err(err) => {
                if !args.silent {
                    error::err(&format!("failed to read file '{}': {}", file.name, err));
                }
                exit(1);
            }
        };
        let mut ignored_errors = 0;
        let mut lexer = Lexer::new(&content, file.name.as_str());
        let toks = lexer.run();
        errors.push(lexer.errors);

        if !toks.is_empty() {
            let mut parser = parser::Parser::new(toks, file.name.as_str());
            let _ = parser.parse();
            errors.push(parser.errors);
        }

        let processed_errors = errors
            .iter()
            .flatten()
            .filter(|e| {
                if config.disabled.rules.contains(&e.rule) {
                    ignored_errors += 1;
                    false
                } else {
                    true
                }
            })
            .collect::<Vec<&error::Error>>();

        if !processed_errors.is_empty() && !args.silent {
            error::print_str_colored(
                &format!("{:=^72}\n", format!(" {} ", file.name)),
                error::Color::Blue,
            );
            let error_count = processed_errors.len();
            for (i, e) in processed_errors.iter().enumerate() {
                (**e).clone().print(&content);
                if i + 1 != error_count {
                    println!()
                }
            }
        }
        file.errors = processed_errors.len();
        file.ignored_errors = ignored_errors;
    }

    if args.silent {
        let verified = files.iter().filter(|f| f.errors == 0).count();
        if verified != files.len() {
            exit(1);
        }
        return;
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
    );

    if verified != files.len() {
        exit(1);
    }
}
