use std::fmt;

use crate::{
    ast::{Command, ShellCommand},
    help::help_text,
    http::{Client, HttpResponse},
    session::Session,
};

pub struct Executor {
    client: Client,
}

pub enum ControlFlow {
    Continue,
    Exit,
}

#[derive(Debug)]
pub enum Output {
    None,
    Text(String),
    HttpResponse(HttpResponse),
}

pub struct ExecutionResult {
    pub control_flow: ControlFlow,
    pub output: Output,
}

#[derive(Debug)]
pub enum ExecutionError {
    HttpError(String),
    SessionError(String),
    VariableError(String),
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecutionError::HttpError(message) => write!(f, "http: {}", message),
            ExecutionError::SessionError(message) => write!(f, "session: {}", message),
            ExecutionError::VariableError(message) => write!(f, "variable: {}", message),
        }
    }
}

impl std::error::Error for ExecutionError {}

impl Executor {
    pub fn new() -> Self {
        Self {
            client: Client::new(),
        }
    }

    pub fn execute(
        &mut self,
        command: Command,
        session: &mut Session,
    ) -> Result<ExecutionResult, ExecutionError> {
        match command {
            Command::Session(command) => {
                session.apply_session(command);
                Ok(ExecutionResult {
                    control_flow: ControlFlow::Continue,
                    output: Output::None,
                })
            }

            Command::Variable(command) => {
                session.apply_variable(command);
                Ok(ExecutionResult {
                    control_flow: ControlFlow::Continue,
                    output: Output::None,
                })
            }

            Command::Shell(command) => Ok(self.execute_shell(command)),

            Command::History(_) => {
                todo!("history")
            }

            Command::Request(_) => {
                todo!("saved requests")
            }

            Command::Http(command) => {
                let request = session
                    .build_request(command)
                    .map_err(|e| ExecutionError::SessionError(e))?;

                let response = self
                    .client
                    .send(request)
                    .map_err(|e| ExecutionError::HttpError(format!("request failed: {}", e)))?;

                Ok(ExecutionResult {
                    control_flow: ControlFlow::Continue,
                    output: Output::HttpResponse(response),
                })
            }
        }
    }

    fn execute_shell(&self, command: ShellCommand) -> ExecutionResult {
        match command {
            ShellCommand::Help => {
                let help_text = help_text();
                ExecutionResult {
                    control_flow: ControlFlow::Continue,
                    output: Output::Text(help_text),
                }
            }

            ShellCommand::Version => {
                let version = env!("CARGO_PKG_VERSION");
                ExecutionResult {
                    control_flow: ControlFlow::Continue,
                    output: Output::Text(version.to_string()),
                }
            }

            ShellCommand::Clear => ExecutionResult {
                control_flow: ControlFlow::Continue,
                output: Output::Text("\x1b[2J\x1b[H".to_string()),
            },

            ShellCommand::Exit => ExecutionResult {
                control_flow: ControlFlow::Exit,
                output: Output::None,
            },
        }
    }
}
