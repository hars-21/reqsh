use std::process;

use rustyline::config::BellStyle;
use rustyline::error::ReadlineError;
use rustyline::{CompletionType, Config, EditMode, Editor};

use reqsh::context::RequestContext;
use reqsh::helper::ShellHelper;
use reqsh::parser::{ShellCommand, ShellSignal};

const HISTORY_FILE: &str = "history.txt";

fn shell_loop() {
    let config = Config::builder()
        .history_ignore_space(true)
        .completion_type(CompletionType::List)
        .edit_mode(EditMode::Vi)
        .bell_style(BellStyle::Audible)
        .build();

    let mut ctx = RequestContext::new();
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

                rl.add_history_entry(&line).unwrap();

                let cmd = ShellCommand::build(line).unwrap_or_else(|err| {
                    eprintln!("Error parsing arguments: {err}");
                    process::exit(1);
                });

                let result = cmd.execute(&mut ctx);
                match result.signal {
                    ShellSignal::Continue => {
                        if let Some(output) = result.output {
                            println!("{}", output);
                        }
                    }
                    ShellSignal::Exit => break,
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
