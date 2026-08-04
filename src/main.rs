use std::env;

use reqsh::{help::help_text, repl::Repl};

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.as_slice() {
        [] => {
            let mut repl = Repl::new();
            repl.run().unwrap_or_else(|err| {
                eprintln!("Error: {err}");
                std::process::exit(1);
            });
        }

        [arg] if arg == "--help" || arg == "-h" => {
            println!("{}", help_text())
        }

        [arg] if arg == "--version" || arg == "-v" => {
            println!("reqsh {}", VERSION);
        }

        [arg, value] if arg == "--timeout" => {
            let secs: u64 = value.parse().unwrap_or_else(|_| {
                eprintln!("Invalid timeout: {value}");
                std::process::exit(1);
            });

            let mut repl = Repl::new_with_timeout(secs);
            repl.run().unwrap_or_else(|err| {
                eprintln!("Error: {err}");
                std::process::exit(1);
            });
        }

        [unknown] => {
            eprintln!("Unknown argument: {}", unknown);
            eprintln!("Try 'reqsh --help'");
            std::process::exit(1);
        }

        _ => {
            eprintln!("Too many arguments");
            eprintln!("Try 'reqsh --help'");
            std::process::exit(1);
        }
    }
}
