use std::fs::OpenOptions;
use std::io::{self, Write};

use crate::context::{RequestContext, RequestMethod};
use crate::{get_request, help};

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
            "GET" => {
                let response = get_request(&self.args[0], ctx);
                writeln!(stdout, "{response}").unwrap();
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
                if self.args.len() == 2 {
                    writeln!(
                        stdout,
                        "Saving request '{}' as '{}'",
                        self.args[1], self.args[0]
                    )
                    .unwrap();
                    ctx.save_request(&self.args[0], RequestMethod::GET, &self.args[1]);
                } else {
                    writeln!(stderr, "Usage: save <request_name> <alias>").unwrap();
                }
            }
            "run" => {
                if self.args.len() == 1 {
                    let url = ctx.get_saved_request(&self.args[0]).map(|r| r.url.clone());
                    if let Some(url) = url {
                        writeln!(stdout, "Running saved request '{}'", self.args[0]).unwrap();
                        let response = get_request(&url, ctx);
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
            _ => {
                writeln!(stdout, "Executing command: {}", self.name).unwrap();
                if !self.args.is_empty() {
                    writeln!(stderr, "With arguments: {:?}", self.args).unwrap();
                }
            }
        }

        ShellSignal::Continue
    }
}
