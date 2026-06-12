use std::env;
use std::path::PathBuf;

use reqsh::builtin::{ControlFlow, handle};
use reqsh::help::help_text;
use reqsh::parser::{Parsed, parse};
use rustyline::error::ReadlineError;
use rustyline::history::FileHistory;
use rustyline::{CompletionType, Config, EditMode, Editor};

use colored::Colorize;
use reqsh::executor::execute;
use reqsh::helper::ShellHelper;
use reqsh::state::ShellState;

const VERSION: &str = env!("CARGO_PKG_VERSION");

fn history_path() -> PathBuf {
    let home = dirs::home_dir().expect("could not determine home directory");
    home.join(".reqsh_history")
}

fn shell_loop() {
    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Vi)
        .build();

    let hist_path = history_path();
    let mut ctx = ShellState::new();
    let mut rl = Editor::with_config(config).unwrap();
    rl.set_helper(Some(ShellHelper));
    rl.load_history(&hist_path).unwrap_or_default();

    loop {
        let readline = rl.readline("reqsh> ");
        match readline {
            Ok(line) => {
                if line.trim().is_empty() {
                    continue;
                }

                let tokens: Vec<&str> = line.split_whitespace().collect();
                let methods = ["GET", "POST", "PUT", "DELETE"];
                let raw = if methods.contains(&tokens[0]) {
                    collect_input(&mut rl, line)
                } else {
                    line
                };

                rl.add_history_entry(&raw).unwrap_or_default();

                match parse(raw) {
                    Ok(parsed) => match parsed {
                        Parsed::Builtin(cmd) => match handle(cmd, &mut ctx, rl.history()) {
                            Ok(ControlFlow::Continue) => {}
                            Ok(ControlFlow::Exit) => {
                                break;
                            }
                            Err(e) => {
                                eprintln!("{}", e.red().bold());
                            }
                        },

                        Parsed::Request(req) => {
                            ctx.set_last_request(req.clone());
                            match execute(req, &ctx) {
                                Ok(res) => {
                                    println!("{}", res);
                                }
                                Err(e) => {
                                    eprintln!("{}", e.red().bold());
                                }
                            }
                        }

                        Parsed::Exit => {
                            println!("{}", "Bye!".dimmed());
                            break;
                        }
                    },

                    Err(e) => {
                        eprintln!("{}", e.red().bold());
                    }
                }
            }

            Err(ReadlineError::Interrupted) => {
                break;
            }

            Err(ReadlineError::Eof) => {
                break;
            }

            Err(err) => {
                eprintln!("Error: {:?}", err);
                break;
            }
        }
    }

    rl.append_history(&hist_path).unwrap_or_default();
}

fn collect_input(rl: &mut Editor<ShellHelper, FileHistory>, first_line: String) -> String {
    let mut buffer = String::new();
    buffer.push_str(&first_line);
    buffer.push('\n');

    loop {
        let inner_rl = rl.readline(".....> ");
        match inner_rl {
            Ok(inner_line) => {
                if inner_line == "::send" {
                    break;
                }

                buffer.push_str(&inner_line);
                buffer.push('\n');
            }

            Err(ReadlineError::Interrupted) => {
                buffer.clear();
                continue;
            }

            Err(ReadlineError::Eof) => {
                break;
            }

            Err(err) => {
                eprintln!("Error: {:?}", err);
                buffer.clear();
                continue;
            }
        }
    }

    buffer
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();

    match args.as_slice() {
        [] => {
            shell_loop();
        }

        [arg] if arg == "--help" || arg == "-h" => {
            println!("{}", help_text())
        }

        [arg] if arg == "--version" || arg == "-v" => {
            println!("reqsh {}", VERSION);
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
