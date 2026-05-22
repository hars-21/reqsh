use reqsh::builtin::handle;
use reqsh::parser::{Parsed, parse};
use rustyline::error::ReadlineError;
use rustyline::history::FileHistory;
use rustyline::{CompletionType, Config, EditMode, Editor};

use colored::Colorize;
use reqsh::executor::execute;
use reqsh::helper::ShellHelper;
use reqsh::state::ShellState;

const HISTORY_FILE: &str = ".reqsh_history";

fn shell_loop() {
    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Vi)
        .build();

    let mut ctx = ShellState::new();
    let mut rl = Editor::with_config(config).unwrap();
    rl.set_helper(Some(ShellHelper));
    rl.load_history(HISTORY_FILE).unwrap_or_default();

    loop {
        let readline = rl.readline("reqsh> ");
        match readline {
            Ok(line) => {
                if line.trim().is_empty() {
                    continue;
                }

                let tokens: Vec<&str> = line.split_whitespace().collect();
                let methods = vec!["GET", "POST", "PUT", "DELETE"];
                let raw;
                if methods.contains(&tokens[0]) {
                    raw = collect_input(&mut rl, line);
                } else {
                    raw = line;
                }

                rl.add_history_entry(&raw).unwrap_or_default();

                match parse(raw) {
                    Ok(parsed) => match parsed {
                        Parsed::Builtin(cmd) => {
                            if let Err(e) = handle(cmd, &mut ctx, rl.history()) {
                                println!("{}", e.red().bold());
                            }
                        }

                        Parsed::Request(req) => match execute(req, &ctx) {
                            Ok(res) => {
                                println!("{}", res);
                            }

                            Err(e) => {
                                println!("{}", e.red().bold());
                            }
                        },

                        Parsed::Exit => {
                            println!("{}", "Bye!".dimmed());
                            break;
                        }
                    },

                    Err(e) => {
                        println!("{}", e.red().bold());
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
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    rl.append_history(HISTORY_FILE).unwrap_or_default();
}

fn collect_input(rl: &mut Editor<ShellHelper, FileHistory>, first_line: String) -> String {
    let mut buffer = String::new();
    buffer.push_str(&first_line);
    buffer.push_str("\n");

    loop {
        let inner_rl = rl.readline(".....> ");
        if let Ok(inner_line) = inner_rl {
            if inner_line == "::send" {
                break;
            }

            buffer.push_str(&inner_line);
            buffer.push_str("\n");
        } else {
            break;
        }
    }

    return buffer;
}

fn main() {
    shell_loop();
}
