use rustyline::history::FileHistory;

use crate::{
    executor::execute,
    help::help_text,
    parser::{Parsed, parse},
    state::ShellState,
};

pub enum Builtin {
    Help,
    Base(String),
    Header(String, String),
    History,
    Rerun(usize),
}

pub fn handle(cmd: Builtin, ctx: &mut ShellState, history: &FileHistory) -> Result<(), String> {
    match cmd {
        Builtin::Base(url) => {
            ctx.set_base_url(&url);
        }

        Builtin::Header(k, v) => {
            ctx.set_header(k, v);
        }

        Builtin::Help => {
            let help = help_text();
            println!("{}", help);
        }

        Builtin::History => {
            for line in history.iter() {
                println!("{}", line);
            }
        }

        Builtin::Rerun(index) => {
            let mut it = 0;
            for line in history.iter() {
                it += 1;
                if it == index {
                    match parse(line.to_string()) {
                        Ok(parsed) => match parsed {
                            Parsed::Builtin(cmd) => {
                                if let Err(e) = handle(cmd, ctx, history) {
                                    println!("{}", e);
                                };
                            }

                            Parsed::Request(req) => {
                                match execute(req, &ctx) {
                                    Ok(c) => println!("{c}"),
                                    Err(e) => println!("{e}"),
                                };
                            }

                            Parsed::Exit => break,
                        },
                        Err(e) => println!("{}", e),
                    }
                }
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base_command_sets_base_url() {
        let mut state = ShellState::new();
        let history = FileHistory::new();
        let cmd = Builtin::Base("https://example.com".to_string());

        let result = handle(cmd, &mut state, &history);

        assert!(result.is_ok());
        assert_eq!(state.get_base_url(), Some("https://example.com"));
    }

    #[test]
    fn header_command_adds_header() {
        let mut state = ShellState::new();
        let history = FileHistory::new();
        let cmd = Builtin::Header("Content-Type".to_string(), "application/json".to_string());

        let result = handle(cmd, &mut state, &history);

        assert!(result.is_ok());
        assert_eq!(
            state.get_headers().get("Content-Type"),
            Some(&"application/json".to_string())
        );
    }
}
