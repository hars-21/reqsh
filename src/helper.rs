use std::cell::RefCell;
use std::rc::Rc;

use rustyline::completion::{Completer, Pair, extract_word};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper, Result};

use crate::state::ShellState;

const BUILTINS: &[&str] = &[
    "GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS", "base", "header", "exit", "help",
    "history", "rerun", "set", "timeout", "unset", "save", "run", "vars", "headers", "requests",
    "clear",
];

pub struct ShellHelper {
    state: Rc<RefCell<ShellState>>,
}

impl ShellHelper {
    pub fn new(state: Rc<RefCell<ShellState>>) -> Self {
        ShellHelper { state }
    }
}

impl Helper for ShellHelper {}
impl Hinter for ShellHelper {
    type Hint = String;
}
impl Highlighter for ShellHelper {}
impl Validator for ShellHelper {}

impl Completer for ShellHelper {
    type Candidate = Pair;

    fn complete(&self, line: &str, pos: usize, _ctx: &Context<'_>) -> Result<(usize, Vec<Pair>)> {
        let (start, word) = extract_word(line, pos, None, |c| c == ' ');
        let before_cursor = line[..start].trim();

        if before_cursor.is_empty() {
            let matches: Vec<Pair> = BUILTINS
                .iter()
                .copied()
                .filter(|cmd| cmd.starts_with(word))
                .map(|cmd| Pair {
                    display: cmd.to_string(),
                    replacement: cmd.to_string(),
                })
                .collect();

            return Ok((start, matches));
        }

        let parts: Vec<&str> = before_cursor.split_whitespace().collect();

        match parts[0] {
            "run" => {
                let state = self.state.borrow();
                let matches: Vec<Pair> = state
                    .get_all_requests()
                    .keys()
                    .filter(|name| name.starts_with(word))
                    .map(|name| Pair {
                        display: name.clone(),
                        replacement: name.clone(),
                    })
                    .collect();
                return Ok((start, matches));
            }
            "unset" if parts.len() == 1 => {
                let state = self.state.borrow();
                let mut matches: Vec<Pair> = state
                    .get_variables()
                    .keys()
                    .filter(|name| name.starts_with(word))
                    .map(|name| Pair {
                        display: name.clone(),
                        replacement: name.clone(),
                    })
                    .collect();
                if "header".starts_with(word) {
                    matches.push(Pair {
                        display: "header".to_string(),
                        replacement: "header".to_string(),
                    });
                }
                return Ok((start, matches));
            }
            "unset" if parts.len() >= 2 && parts[1] == "header" => {
                let state = self.state.borrow();
                let matches: Vec<Pair> = state
                    .get_headers()
                    .keys()
                    .filter(|name| name.starts_with(word))
                    .map(|name| Pair {
                        display: name.clone(),
                        replacement: name.clone(),
                    })
                    .collect();
                return Ok((start, matches));
            }
            "header" => {
                let state = self.state.borrow();
                let matches: Vec<Pair> = state
                    .get_headers()
                    .keys()
                    .filter(|name| name.starts_with(word))
                    .map(|name| Pair {
                        display: name.clone(),
                        replacement: name.clone(),
                    })
                    .collect();
                return Ok((start, matches));
            }
            _ => {}
        }

        Ok((start, vec![]))
    }
}
