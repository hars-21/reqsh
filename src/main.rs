use std::cell::RefCell;
use std::env;
use std::path::PathBuf;
use std::rc::Rc;

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

fn run_repl(ctx: Rc<RefCell<ShellState>>) {
    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Vi)
        .build();

    let hist_path = history_path();
    let mut rl = Editor::with_config(config).unwrap();
    rl.set_helper(Some(ShellHelper::new(ctx.clone())));
    rl.load_history(&hist_path).unwrap_or_default();

    loop {
        let readline = rl.readline("reqsh> ");
        match readline {
            Ok(line) => {
                if line.trim().is_empty() {
                    continue;
                }

                let tokens: Vec<&str> = line.split_whitespace().collect();
                let methods = ["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"];
                let raw = if methods.contains(&tokens[0]) {
                    collect_input(&mut rl, line)
                } else {
                    line
                };

                rl.add_history_entry(&raw).unwrap_or_default();
                rl.save_history(&hist_path).unwrap_or_default();

                match parse(raw) {
                    Ok(parsed) => match parsed {
                        Parsed::Builtin(cmd) => {
                            let mut state = ctx.borrow_mut();
                            match handle(cmd, &mut state, rl.history()) {
                                Ok(ControlFlow::Continue) => {}
                                Ok(ControlFlow::Exit) => {
                                    break;
                                }
                                Err(e) => {
                                    eprintln!("{}", e.red().bold());
                                }
                            }
                        }

                        Parsed::Request(req) => {
                            ctx.borrow_mut().set_last_request(req.clone());
                            let state = ctx.borrow();
                            match execute(req, &state) {
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
                println!("^C");
                continue;
            }

            Err(ReadlineError::Eof) => {
                println!();
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
            let ctx = Rc::new(RefCell::new(ShellState::new()));
            run_repl(ctx);
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
            let ctx = Rc::new(RefCell::new(ShellState::new()));
            ctx.borrow_mut().set_timeout(secs);
            run_repl(ctx);
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
