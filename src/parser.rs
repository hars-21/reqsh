use std::fs::OpenOptions;
use std::io::{self, Write};

use crate::context::RequestContext;
use crate::help;
use crate::request::{Request, RequestMethod};

pub struct ShellCommand {
    pub name: String,
    pub args: Vec<String>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub append: bool,
}

enum RedirectType {
    Stdout,
    Stderr,
}

pub enum ShellSignal {
    Continue,
    Exit,
}

impl ShellCommand {
    pub fn build(command_line: &str) -> Result<ShellCommand, &str> {
        let mut tokens = Vec::new();
        let mut current = String::new();
        let mut chars = command_line.chars().peekable();
        let mut pending_redirect: Option<RedirectType> = None;
        let mut stdout: Option<String> = None;
        let mut stderr: Option<String> = None;
        let mut append = false;

        while let Some(c) = chars.next() {
            match c {
                '\'' => {
                    while let Some(c) = chars.next() {
                        if c == '\'' {
                            break;
                        } else {
                            current.push(c);
                        }
                    }
                }

                '"' => {
                    while let Some(c) = chars.next() {
                        if c == '"' {
                            break;
                        }
                        if c == '\\' {
                            if let Some(next) = chars.next() {
                                match next {
                                    '"' | '\\' | '$' | '\n' => current.push(next),
                                    _ => {
                                        current.push('\\');
                                        current.push(next);
                                    }
                                }
                            }
                        } else {
                            current.push(c);
                        }
                    }
                }

                '\\' => {
                    if let Some(next) = chars.next() {
                        current.push(next);
                    }
                }

                ' ' | '\t' => {
                    if !current.is_empty() {
                        if let Some(rtype) = pending_redirect.take() {
                            match rtype {
                                RedirectType::Stdout => stdout = Some(current.clone()),
                                RedirectType::Stderr => stderr = Some(current.clone()),
                            }
                        } else {
                            tokens.push(current.clone());
                        }
                        current.clear();
                    }
                }

                '1' => {
                    if let Some('>') = chars.peek() {
                        chars.next();
                        pending_redirect = Some(RedirectType::Stdout);
                        if let Some('>') = chars.peek() {
                            chars.next();
                            append = true;
                        }
                    } else {
                        current.push('1');
                    }
                }

                '2' => {
                    if let Some('>') = chars.peek() {
                        chars.next();
                        pending_redirect = Some(RedirectType::Stderr);
                        if let Some('>') = chars.peek() {
                            chars.next();
                            append = true;
                        }
                    } else {
                        current.push('2');
                    }
                }

                '>' => {
                    if let Some('>') = chars.peek() {
                        chars.next();
                        pending_redirect = Some(RedirectType::Stdout);
                        append = true;
                    } else {
                        pending_redirect = Some(RedirectType::Stdout);
                    }
                }

                _ => current.push(c),
            }
        }

        if !current.is_empty() {
            if let Some(rtype) = pending_redirect.take() {
                match rtype {
                    RedirectType::Stdout => stdout = Some(current.clone()),
                    RedirectType::Stderr => stderr = Some(current.clone()),
                }
            } else {
                tokens.push(current.clone());
            }
            current.clear();
        }

        let (name, args) = tokens.split_first().unwrap();

        Ok(ShellCommand {
            name: name.clone(),
            args: args.to_vec(),
            stdout,
            stderr,
            append,
        })
    }

    pub fn execute(&self, ctx: &mut RequestContext) -> ShellSignal {
        let mut stdout: Box<dyn Write> = match &self.stdout {
            Some(file) => Box::new(
                OpenOptions::new()
                    .create(true)
                    .write(true)
                    .append(self.append)
                    .open(file)
                    .unwrap(),
            ),
            None => Box::new(io::stdout()),
        };

        let mut stderr: Box<dyn Write> = match &self.stderr {
            Some(file) => Box::new(
                OpenOptions::new()
                    .create(true)
                    .write(true)
                    .append(self.append)
                    .open(file)
                    .unwrap(),
            ),
            None => Box::new(io::stderr()),
        };

        match self.name.as_str() {
            "exit" => return ShellSignal::Exit,

            "help" => {
                writeln!(stdout, "{}", help::get_help()).unwrap();
            }

            "set" => {
                if self.args.len() == 2 && self.args[0] == "base_url" {
                    ctx.set_base_url(&self.args[1]);
                    writeln!(stdout, "Base URL set to: {}", self.args[1]).unwrap();
                } else {
                    writeln!(stderr, "Usage: set base_url <url>").unwrap();
                }
            }

            "save" => {
                if self.args.len() == 3 {
                    writeln!(stdout, "Saved request '{}'", self.args[0]).unwrap();
                    ctx.save_request(&self.args[0], RequestMethod::GET, &self.args[2]);
                } else {
                    writeln!(stderr, "Usage: save <request_name> <method> <url>").unwrap();
                }
            }

            "run" => {
                if self.args.len() == 1 {
                    if let Some(request) = ctx.get_saved_request(&self.args[0]) {
                        let response = request.fetch(ctx.get_base_url());
                        writeln!(stdout, "{response}").unwrap();
                    } else {
                        writeln!(
                            stderr,
                            "No saved request found with name '{}'",
                            self.args[0]
                        )
                        .unwrap();
                    }
                } else {
                    writeln!(stderr, "Usage: run <request_name>").unwrap();
                }
            }

            "list" => {
                writeln!(stdout, "Saved requests:").unwrap();
                for name in ctx.list_saved_requests() {
                    writeln!(stdout, "  {}", name).unwrap();
                }
            }

            "delete" => {
                if self.args.len() == 1 {
                    if ctx.delete_saved_request(&self.args[0]) {
                        writeln!(stdout, "Deleted saved request '{}'", self.args[0]).unwrap();
                    } else {
                        writeln!(
                            stderr,
                            "No saved request found with name '{}'",
                            self.args[0]
                        )
                        .unwrap();
                    }
                } else {
                    writeln!(stderr, "Usage: delete <request_name>").unwrap();
                }
            }

            "GET" => {
                if self.args.len() == 1 {
                    let request = Request::new(RequestMethod::GET, self.args[0].clone());
                    let response = request.fetch(ctx.get_base_url());
                    writeln!(stdout, "{}", response).unwrap();
                } else {
                    writeln!(stderr, "Usage: GET <url>").unwrap();
                }
            }

            _ => {
                writeln!(stdout, "Command not found: {}", self.name).unwrap();
            }
        }

        ShellSignal::Continue
    }
}
