use std::time::Duration;

use crate::{
    ast::{
        Command, Header, HeaderCommand, HistoryCommand, Method, Query, RequestCommand, RequestSpec,
        SessionCommand, ShellCommand, VariableCommand,
    },
    lexer::{Lexer, LexerError, Token},
};

pub fn parse(input: &str) -> Result<Command, String> {
    let tokens = match Lexer::lex(input) {
        Ok(tokens) => tokens,
        Err(LexerError::UnterminatedString) => {
            return Err("error: unterminated string".into());
        }
        Err(LexerError::InvalidVariable) => {
            return Err("error: invalid variable".into());
        }
    };

    if tokens.is_empty() {
        return Err("empty input".into());
    }

    let command = word(&tokens, 0)?.to_ascii_lowercase();

    match command.as_str() {
        "get" | "post" | "put" | "patch" | "delete" | "head" | "options" => parse_request(&tokens),

        "base" | "header" | "timeout" => parse_session(&tokens),

        "var" => parse_variable(&tokens),

        "history" => parse_history(&tokens),

        "req" => parse_saved_request(&tokens),

        "help" => Ok(Command::Shell(ShellCommand::Help)),

        "version" => Ok(Command::Shell(ShellCommand::Version)),

        "clear" => Ok(Command::Shell(ShellCommand::Clear)),

        "exit" => Ok(Command::Shell(ShellCommand::Exit)),

        other => Err(format!("unknown command: {}", other)),
    }
}

fn word(tokens: &[Token], index: usize) -> Result<&str, String> {
    match tokens.get(index) {
        Some(Token::Word(word)) => Ok(word),
        _ => Err(format!("expected word at token {}", index)),
    }
}

fn string(tokens: &[Token], index: usize) -> Result<String, String> {
    match tokens.get(index) {
        Some(Token::Word(value)) => Ok(value.clone()),
        Some(Token::String(value)) => Ok(value.clone()),
        Some(Token::Variable(value)) => Ok(format!("{{{{{}}}}}", value)),
        _ => Err(format!("expected value at token {}", index)),
    }
}

fn method(word: &str) -> Option<Method> {
    match word.to_ascii_lowercase().as_str() {
        "get" => Some(Method::Get),
        "post" => Some(Method::Post),
        "put" => Some(Method::Put),
        "patch" => Some(Method::Patch),
        "delete" => Some(Method::Delete),
        "head" => Some(Method::Head),
        "options" => Some(Method::Options),
        _ => None,
    }
}

fn parse_session(tokens: &[Token]) -> Result<Command, String> {
    match word(tokens, 0)?.to_ascii_lowercase().as_str() {
        "base" => {
            let url = string(tokens, 1).map_err(|_| "usage: base <url>".to_string())?;

            Ok(Command::Session(SessionCommand::Base(url)))
        }

        "timeout" => {
            let duration = word(tokens, 1).map_err(|_| "usage: timeout <seconds>".to_string())?;

            let seconds = duration
                .parse::<u64>()
                .map_err(|_| "invalid timeout".to_string())?;

            Ok(Command::Session(SessionCommand::Timeout(
                Duration::from_secs(seconds),
            )))
        }

        "header" => {
            let sub = word(tokens, 1)?.to_ascii_lowercase();

            let cmd = match sub.as_str() {
                "set" => HeaderCommand::Set {
                    name: string(tokens, 2)?,
                    value: string(tokens, 3)?,
                },

                "list" => HeaderCommand::List,

                "remove" => HeaderCommand::Remove {
                    name: string(tokens, 2)?,
                },

                "clear" => HeaderCommand::Clear,

                _ => return Err(format!("unknown header command: {}", sub)),
            };

            Ok(Command::Session(SessionCommand::Header(cmd)))
        }

        _ => unreachable!(),
    }
}

fn parse_variable(tokens: &[Token]) -> Result<Command, String> {
    let sub = word(tokens, 1)?.to_ascii_lowercase();

    let cmd = match sub.as_str() {
        "set" => VariableCommand::Set {
            name: string(tokens, 2)?,
            value: string(tokens, 3)?,
        },

        "list" => VariableCommand::List,

        "remove" => VariableCommand::Remove {
            name: string(tokens, 2)?,
        },

        "clear" => VariableCommand::Clear,

        _ => return Err(format!("unknown variable command: {}", sub)),
    };

    Ok(Command::Variable(cmd))
}

fn parse_history(tokens: &[Token]) -> Result<Command, String> {
    if tokens[1] == Token::Newline {
        return Ok(Command::History(HistoryCommand::List));
    }

    let sub = word(tokens, 1)?.to_ascii_lowercase();

    let cmd = match sub.as_str() {
        "list" => HistoryCommand::List,

        "show" => HistoryCommand::Show {
            index: parse_index(word(tokens, 2)?)?,
        },

        "remove" => HistoryCommand::Remove {
            index: parse_index(word(tokens, 2)?)?,
        },

        "rerun" => HistoryCommand::Rerun {
            index: parse_index(word(tokens, 2)?)?,
        },

        "clear" => HistoryCommand::Clear,

        _ => return Err(format!("unknown history command: {}", sub)),
    };

    Ok(Command::History(cmd))
}

fn parse_saved_request(tokens: &[Token]) -> Result<Command, String> {
    let sub = word(tokens, 1)?.to_ascii_lowercase();

    let cmd = match sub.as_str() {
        "save" => RequestCommand::Save {
            name: string(tokens, 2)?,
        },

        "run" => RequestCommand::Run {
            name: string(tokens, 2)?,
        },

        "list" => RequestCommand::List,

        "show" => RequestCommand::Show {
            name: string(tokens, 2)?,
        },

        "rename" => RequestCommand::Rename {
            old_name: string(tokens, 2)?,
            new_name: string(tokens, 3)?,
        },

        "remove" => RequestCommand::Remove {
            name: string(tokens, 2)?,
        },

        "clear" => RequestCommand::Clear,

        _ => return Err(format!("unknown request command: {}", sub)),
    };

    Ok(Command::Request(cmd))
}

fn parse_index(value: &str) -> Result<usize, String> {
    value
        .parse()
        .map_err(|_| format!("invalid index: {}", value))
}

fn parse_request(tokens: &[Token]) -> Result<Command, String> {
    let method = method(word(tokens, 0)?).ok_or_else(|| "invalid http method".to_string())?;

    let path = string(tokens, 1)?;

    let mut headers = Vec::new();
    let mut queries = Vec::new();
    let mut body = String::new();

    let mut i = 2;
    let mut in_body = false;

    while i < tokens.len() {
        match &tokens[i] {
            Token::End => break,

            Token::Newline => {
                if in_body {
                    body.push('\n');
                } else {
                    in_body = true;
                }
                i += 1;
            }

            _ if in_body => {
                body.push_str(&collect_until_newline(tokens, &mut i)?);
            }

            Token::Word(name) => match tokens.get(i + 1) {
                Some(Token::Colon) => {
                    let value = collect_value(tokens, &mut i, 2)?;
                    headers.push(Header {
                        name: name.clone(),
                        value,
                    });
                }

                Some(Token::Equals) => {
                    let value = collect_value(tokens, &mut i, 2)?;
                    queries.push(Query {
                        name: name.clone(),
                        value,
                    });
                }

                _ => {
                    return Err(format!("unexpected token after '{}'", name));
                }
            },

            token => {
                return Err(format!("unexpected token: {:?}", token));
            }
        }
    }

    Ok(Command::Http(RequestSpec {
        method,
        path,
        headers,
        queries,
        body,
    }))
}

fn collect_until_newline(tokens: &[Token], index: &mut usize) -> Result<String, String> {
    let mut text = String::new();

    while *index < tokens.len() {
        match &tokens[*index] {
            Token::Newline | Token::End => break,

            Token::Word(s) => {
                if !text.is_empty() {
                    text.push(' ');
                }
                text.push_str(s);
            }

            Token::String(s) => {
                if !text.is_empty() {
                    text.push(' ');
                }
                text.push_str(s);
            }

            Token::Variable(v) => {
                if !text.is_empty() {
                    text.push(' ');
                }

                text.push_str("{{");
                text.push_str(v);
                text.push_str("}}");
            }

            Token::Colon => text.push(':'),
            Token::Equals => text.push('='),
        }

        *index += 1;
    }

    Ok(text)
}

fn collect_value(tokens: &[Token], index: &mut usize, offset: usize) -> Result<String, String> {
    *index += offset;

    let value = collect_until_newline(tokens, index)?;

    if matches!(tokens.get(*index), Some(Token::Newline)) {
        *index += 1;
    }

    Ok(value)
}
