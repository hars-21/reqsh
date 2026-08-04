use std::fmt;

use super::ast::*;
use super::lexer::Token;

#[derive(Debug)]
pub enum ParserError {
    EmptyInput,
    InvalidCommand,
    MissingArgument,
    InvalidMethod,
}

impl fmt::Display for ParserError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ParserError::EmptyInput => {
                write!(f, "empty command")
            }

            ParserError::InvalidCommand => {
                write!(f, "invalid command")
            }

            ParserError::MissingArgument => {
                write!(f, "missing required argument")
            }

            ParserError::InvalidMethod => {
                write!(f, "invalid HTTP method")
            }
        }
    }
}

impl std::error::Error for ParserError {}

pub struct Parser;

impl Parser {
    pub fn parse(tokens: Vec<Token>) -> Result<Command, ParserError> {
        let mut parser = ParserState {
            tokens,
            position: 0,
        };

        parser.parse_command()
    }
}

struct ParserState {
    tokens: Vec<Token>,
    position: usize,
}

impl ParserState {
    fn parse_command(&mut self) -> Result<Command, ParserError> {
        let first = self.next_string().ok_or(ParserError::EmptyInput)?;

        match first.to_ascii_uppercase().as_str() {
            // HTTP commands
            "GET" | "POST" | "PUT" | "DELETE" | "PATCH" | "HEAD" | "OPTIONS" => {
                return self.parse_http(first);
            }

            _ => {}
        }

        match first.as_str() {
            // Session commands
            "base" | "timeout" | "header" => self.parse_session(first),

            // Variable commands
            "var" => self.parse_variable(),

            // Saved requests
            "request" | "req" => self.parse_request_command(),

            // History commands
            "history" => self.parse_history(),

            // Shell commands
            "help" | "version" | "clear" | "exit" => self.parse_shell(first),

            _ => Err(ParserError::InvalidCommand),
        }
    }

    fn parse_http(&mut self, method: String) -> Result<Command, ParserError> {
        let path = self.collect("").ok_or(ParserError::MissingArgument)?;

        let method = Method::from_string(&method).ok_or(ParserError::InvalidMethod)?;

        let mut headers = Vec::new();
        let mut queries = Vec::new();
        let mut body = String::new();

        while let Some(token) = self.peek() {
            match token {
                Token::End => break,

                Token::Newline => {
                    self.advance();
                }

                Token::Body(content) => {
                    body = content.clone();
                    self.advance();

                    break;
                }

                Token::Word(word) => {
                    if let Some(name) = word.strip_suffix(':') {
                        let name = name.trim().to_string();
                        self.advance();

                        let value = self.collect(" ").unwrap_or_default();

                        headers.push(Header { name, value });
                    } else if word.contains('=') {
                        let value = self.collect("").unwrap_or_default();

                        let Some((name, value)) = value.split_once('=') else {
                            return Err(ParserError::InvalidCommand);
                        };

                        queries.push(Query {
                            name: name.to_string(),
                            value: value.to_string(),
                        });
                    } else {
                        return Err(ParserError::InvalidCommand);
                    }
                }

                _ => return Err(ParserError::InvalidCommand),
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

    fn parse_session(&mut self, command: String) -> Result<Command, ParserError> {
        match command.as_str() {
            "base" => {
                let url = self.next_string().ok_or(ParserError::MissingArgument)?;

                Ok(Command::Session(SessionCommand::Base(url)))
            }

            "timeout" => {
                let seconds = self
                    .next_string()
                    .ok_or(ParserError::MissingArgument)?
                    .parse::<u64>()
                    .map_err(|_| ParserError::MissingArgument)?;

                Ok(Command::Session(SessionCommand::Timeout(
                    std::time::Duration::from_secs(seconds),
                )))
            }

            "header" => self.parse_header_command(),

            _ => Err(ParserError::InvalidCommand),
        }
    }

    fn parse_header_command(&mut self) -> Result<Command, ParserError> {
        let action = self.next_string().ok_or(ParserError::MissingArgument)?;

        let command = match action.as_str() {
            "set" => {
                let name = self.next_string().ok_or(ParserError::MissingArgument)?;

                let value = self.collect(" ").ok_or(ParserError::MissingArgument)?;

                HeaderCommand::Set { name, value }
            }

            "list" => HeaderCommand::List,

            "clear" => HeaderCommand::Clear,

            "remove" => {
                let name = self.next_string().ok_or(ParserError::MissingArgument)?;

                HeaderCommand::Remove { name }
            }

            _ => return Err(ParserError::InvalidCommand),
        };

        Ok(Command::Session(SessionCommand::Header(command)))
    }

    fn parse_variable(&mut self) -> Result<Command, ParserError> {
        let action = self.next_string().ok_or(ParserError::MissingArgument)?;

        match action.as_str() {
            "set" => {
                let name = self.next_string().ok_or(ParserError::MissingArgument)?;

                let value = self.next_string().ok_or(ParserError::MissingArgument)?;

                Ok(Command::Variable(VariableCommand::Set { name, value }))
            }

            "list" => Ok(Command::Variable(VariableCommand::List)),

            "clear" => Ok(Command::Variable(VariableCommand::Clear)),

            "remove" => {
                let name = self.next_string().ok_or(ParserError::MissingArgument)?;

                Ok(Command::Variable(VariableCommand::Remove { name }))
            }

            _ => Err(ParserError::InvalidCommand),
        }
    }

    fn parse_request_command(&mut self) -> Result<Command, ParserError> {
        let action = self.next_string().ok_or(ParserError::MissingArgument)?;

        let command = match action.as_str() {
            "save" => {
                let name = self.next_string().ok_or(ParserError::MissingArgument)?;

                RequestCommand::Save { name }
            }

            "run" => {
                let name = self.next_string().ok_or(ParserError::MissingArgument)?;

                RequestCommand::Run { name }
            }

            "show" => {
                let name = self.next_string().ok_or(ParserError::MissingArgument)?;

                RequestCommand::Show { name }
            }

            "remove" => {
                let name = self.next_string().ok_or(ParserError::MissingArgument)?;

                RequestCommand::Remove { name }
            }

            "rename" => {
                let old_name = self.next_string().ok_or(ParserError::MissingArgument)?;

                let new_name = self.next_string().ok_or(ParserError::MissingArgument)?;

                RequestCommand::Rename { old_name, new_name }
            }

            "list" => RequestCommand::List,

            "clear" => RequestCommand::Clear,

            _ => return Err(ParserError::InvalidCommand),
        };

        Ok(Command::Request(command))
    }

    fn parse_history(&mut self) -> Result<Command, ParserError> {
        let command = match self.next_string().as_deref() {
            None | Some("list") => HistoryCommand::List,

            Some("show") => HistoryCommand::Show {
                index: self.parse_index()?,
            },

            Some("clear") => HistoryCommand::Clear,

            Some("rerun") => HistoryCommand::Rerun {
                index: self.parse_index()?,
            },

            _ => return Err(ParserError::InvalidCommand),
        };

        Ok(Command::History(command))
    }

    fn parse_index(&mut self) -> Result<usize, ParserError> {
        self.next_string()
            .ok_or(ParserError::MissingArgument)?
            .parse::<usize>()
            .map_err(|_| ParserError::MissingArgument)
    }

    fn parse_shell(&mut self, command: String) -> Result<Command, ParserError> {
        let cmd = match command.as_str() {
            "help" => ShellCommand::Help,

            "version" => ShellCommand::Version,

            "clear" => ShellCommand::Clear,

            "exit" => ShellCommand::Exit,

            _ => return Err(ParserError::InvalidCommand),
        };

        Ok(Command::Shell(cmd))
    }

    fn next_string(&mut self) -> Option<String> {
        loop {
            match self.advance()? {
                Token::Word(value) | Token::String(value) => {
                    return Some(value);
                }

                Token::Variable(name) => {
                    return Some(format_variable(&name));
                }

                Token::Newline => continue,

                _ => {}
            }
        }
    }

    fn collect(&mut self, separator: &str) -> Option<String> {
        let mut parts = Vec::new();

        while let Some(token) = self.peek() {
            match token {
                Token::Word(value) | Token::String(value) => {
                    parts.push(value.clone());
                    self.advance();
                }

                Token::Variable(name) => {
                    parts.push(format_variable(&name));
                    self.advance();
                }

                _ => break,
            }
        }

        if parts.is_empty() {
            None
        } else {
            Some(parts.join(separator))
        }
    }

    fn peek(&self) -> Option<&Token> {
        self.tokens.get(self.position)
    }

    fn advance(&mut self) -> Option<Token> {
        let token = self.tokens.get(self.position)?.clone();

        self.position += 1;

        Some(token)
    }
}

impl Method {
    fn from_string(value: &str) -> Option<Self> {
        match value.to_ascii_uppercase().as_str() {
            "GET" => Some(Method::Get),

            "POST" => Some(Method::Post),

            "PUT" => Some(Method::Put),

            "DELETE" => Some(Method::Delete),

            "PATCH" => Some(Method::Patch),

            "HEAD" => Some(Method::Head),

            "OPTIONS" => Some(Method::Options),

            _ => None,
        }
    }
}

fn format_variable(name: &str) -> String {
    format!("{{{{{name}}}}}")
}
