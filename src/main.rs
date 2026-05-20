use reqsh::parser::{InputType, parse_command, parse_request};
use rustyline::error::ReadlineError;
use rustyline::{CompletionType, Config, EditMode, Editor};

use reqsh::executor::{execute_command, execute_request};
use reqsh::helper::ShellHelper;
use reqsh::state::State;

const HISTORY_FILE: &str = ".reqsh_history";

fn shell_loop() {
    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Vi)
        .build();

    let mut ctx = State::new();
    let mut rl = Editor::with_config(config).unwrap();
    rl.set_helper(Some(ShellHelper));
    rl.load_history(HISTORY_FILE).unwrap_or_default();

    loop {
        let readline = rl.readline("reqsh> ");
        match readline {
            Ok(line) => {
                if line.is_empty() {
                    continue;
                }

                match InputType::get(&line) {
                    InputType::Request => {
                        let mut buffer = String::new();
                        buffer.push_str(&line);
                        buffer.push_str("\n");
                        loop {
                            let inner_rl = rl.readline("reqsh> ");
                            if let Ok(inner_line) = inner_rl {
                                buffer.push_str(&inner_line);
                                buffer.push_str("\n");

                                if buffer.ends_with("\n\n\n") {
                                    break;
                                }
                            } else {
                                break;
                            }
                        }

                        rl.add_history_entry(&buffer).unwrap();

                        match parse_request(buffer) {
                            Ok(request) => {
                                let result = execute_request(request, &mut ctx);
                                match result {
                                    Ok(c) => println!("{}", c),
                                    Err(e) => println!("{}", e),
                                }
                            }

                            Err(e) => println!("{}", e),
                        }
                    }

                    InputType::Command => {
                        rl.add_history_entry(&line).unwrap();
                        match parse_command(&line) {
                            Ok(cmd) => {
                                if let Err(e) = execute_command(cmd, &mut ctx) {
                                    println!("{}", e);
                                }
                            }

                            Err(e) => println!("{}", e),
                        }
                    }

                    InputType::Error(e) => {
                        println!("{e}")
                    }

                    InputType::Exit => {
                        break;
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

fn main() {
    shell_loop();
}
