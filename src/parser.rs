enum RedirectType {
    Stdout,
    Stderr,
}

pub struct ShellCommand {
    pub name: String,
    pub args: Vec<String>,
    pub stdout: Option<String>,
    pub stderr: Option<String>,
    pub append: bool,
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
}
