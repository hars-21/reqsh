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
    Set(String, String),
    Save(String),
    Run(String),
    UnsetVariable(String),
    UnsetHeader(String),
    Headers,
    Vars,
    Requests,
    Remove(String),
    Clear,
    Timeout(u64),
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

        Builtin::Set(name, value) => {
            ctx.set_variable(name, value);
        }

        Builtin::Header(k, v) => {
            ctx.set_header(k, v);
        }

        Builtin::Headers => {
            for header in ctx.get_headers().iter() {
                println!("{}: {}", header.0, header.1);
            }
        }

        Builtin::Vars => {
            for var in ctx.get_variables().iter() {
                println!("{} = {}", var.0, var.1);
            }
        }

        Builtin::Requests => {
            for (name, req) in ctx.get_all_requests() {
                println!("{} ({}) {}", name, req.method.as_str(), req.path);
            }
        }

        Builtin::Remove(name) => {
            ctx.remove_request(&name)?;
        }

        Builtin::Save(name) => {
            ctx.save_request(name)?;
        }

        Builtin::Run(name) => {
            let req = ctx
                .get_request(&name)
                .ok_or_else(|| format!("no saved request: {name}"))?
                .clone();
            let response = execute(req, ctx).map_err(|e| e.to_string())?;
            println!("{response}");
        }

        Builtin::UnsetVariable(name) => {
            ctx.remove_variable(&name);
        }

        Builtin::UnsetHeader(key) => {
            ctx.remove_header(&key);
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

        Builtin::Timeout(secs) => {
            ctx.set_timeout(secs);
            println!("Request timeout set to {secs}s");
        }

        Builtin::Clear => {
            ctx.clear();
            println!("Session state cleared");
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
