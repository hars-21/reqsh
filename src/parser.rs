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

pub struct ShellOutput {
    pub signal: ShellSignal,
    pub output: Option<String>,
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
    pub fn build(command_line: String) -> Result<ShellCommand, String> {
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
            name: name.to_owned(),
            args: args.to_owned(),
            stdout,
            stderr,
            append,
        })
    }

    pub fn execute(&self, ctx: &mut RequestContext) -> ShellOutput {
        let mut output;

        match self.name.as_str() {
            "exit" => {
                return ShellOutput {
                    signal: ShellSignal::Exit,
                    output: None,
                };
            }

            "help" => {
                output = format!("{}", help::get_help());
            }

            "set" => {
                if self.args.len() == 2 && self.args[0] == "base_url" {
                    ctx.set_base_url(&self.args[1]);
                    output = format!("Base URL set to: {}", self.args[1]);
                } else {
                    output = format!("Usage: set base_url <url>");
                }
            }

            "save" => {
                if self.args.len() == 3 {
                    output = format!("Saved request '{}'", self.args[0]);
                    let method = match self.args[1].as_str() {
                        "GET" => RequestMethod::GET,
                        "POST" => RequestMethod::POST,
                        "PUT" => RequestMethod::PUT,
                        "DELETE" => RequestMethod::DELETE,
                        _ => RequestMethod::GET,
                    };

                    ctx.save_request(&self.args[0], method, &self.args[2]);
                } else {
                    output = format!("Usage: save <request_name> <method> <url>");
                }
            }

            "run" => {
                if self.args.len() == 1 {
                    if let Some(request) = ctx.get_saved_request(&self.args[0]) {
                        let response = request.fetch(ctx.get_base_url());
                        output = format!("{response}");
                    } else {
                        output = format!("No saved request found with name '{}'", self.args[0]);
                    }
                } else {
                    output = format!("Usage: run <request_name>");
                }
            }

            "list" => {
                output = format!("Saved requests:\n");
                for name in ctx.list_saved_requests() {
                    output.push_str(&format!("  {}\n", name));
                }
            }

            "delete" => {
                if self.args.len() == 1 {
                    if ctx.delete_saved_request(&self.args[0]) {
                        output = format!("Deleted saved request '{}'", self.args[0]);
                    } else {
                        output = format!("No saved request found with name '{}'", self.args[0]);
                    }
                } else {
                    output = format!("Usage: delete <request_name>");
                }
            }

            "GET" => {
                if self.args.len() == 1 {
                    let request = Request::new(RequestMethod::GET, self.args[0].clone(), None);
                    let response = request.fetch(ctx.get_base_url());
                    output = format!("{}", response);
                } else {
                    output = format!("Usage: GET <url>");
                }
            }

            "POST" => {
                if self.args.len() >= 1 {
                    let request = Request::new(
                        RequestMethod::POST,
                        self.args[0].clone(),
                        Some(self.args[1].clone()),
                    );
                    let response = request.fetch(ctx.get_base_url());
                    output = format!("{}", response);
                } else {
                    output = format!("Usage POST <url> <body>");
                }
            }

            _ => {
                output = format!("ReferenceError: {} is not defined", self.name);
            }
        }

        ShellOutput {
            signal: ShellSignal::Continue,
            output: Some(output),
        }
    }
}
