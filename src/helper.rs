use rustyline::completion::{Completer, Pair, extract_word};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::{Context, Helper, Result};

const BUILTINS: &[&str] = &[
    "GET", "POST", "PUT", "PATCH", "DELETE", "base", "header", "exit", "help", "history", "rerun",
    "set", "unset", "save", "run", "vars", "headers", "requests",
];

pub struct ShellHelper;

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
        let first_word = line[..start].trim().is_empty();

        if first_word {
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

        Ok((start, vec![]))
    }
}
