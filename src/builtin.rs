use rustyline::history::{FileHistory, History, SearchDirection};

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

pub enum ControlFlow {
    Continue,
    Exit,
}

pub fn handle(
    cmd: Builtin,
    ctx: &mut ShellState,
    history: &FileHistory,
) -> Result<ControlFlow, String> {
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
            for (id, line) in history.iter().enumerate() {
                println!("{:>4}: {}", id + 1, line.trim());
            }
        }

        Builtin::Rerun(index) => {
            if index == 0 {
                return Err("history indices start at 1".into());
            }

            let line = history
                .get(index - 1, SearchDirection::Forward)
                .map_err(|e| e.to_string())?
                .ok_or_else(|| format!("history entry not found: {}", index))?;

            let parsed = parse(line.entry.to_string())?;

            match parsed {
                Parsed::Builtin(cmd) => {
                    handle(cmd, ctx, history)?;
                }

                Parsed::Request(req) => {
                    let response = execute(req, ctx).map_err(|e| e.to_string())?;

                    println!("{response}");
                }

                Parsed::Exit => return Ok(ControlFlow::Exit),
            }
        }
    }

    Ok(ControlFlow::Continue)
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
