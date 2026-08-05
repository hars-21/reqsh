use std::fmt;

use crate::{
    ast::{
        Command, HeaderCommand, RequestCommand, RequestSpec, SessionCommand, ShellCommand,
        VariableCommand,
    },
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

impl Default for Executor {
    fn default() -> Self {
        Self::new()
    }
}

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
            Command::Session(command) => self.execute_session(command, session),

            Command::Variable(command) => Ok(self.execute_variable(command, session)),

            Command::Shell(command) => Ok(self.execute_shell(command)),

            Command::History(_) => Ok(ExecutionResult {
                control_flow: ControlFlow::Continue,
                output: Output::None,
            }),

            Command::Request(command) => self.execute_request(command, session),

            Command::Http(spec) => {
                *session.last_request() = Some(spec.clone());
                self.send_spec(spec, session)
            }
        }
    }

    fn execute_session(
        &self,
        command: SessionCommand,
        session: &mut Session,
    ) -> Result<ExecutionResult, ExecutionError> {
        let result = match command {
            SessionCommand::Base(url) => {
                *session.base_url() = Some(url);

                ExecutionResult {
                    control_flow: ControlFlow::Continue,
                    output: Output::None,
                }
            }

            SessionCommand::Timeout(duration) => {
                *session.timeout() = Some(duration);

                ExecutionResult {
                    control_flow: ControlFlow::Continue,
                    output: Output::None,
                }
            }

            SessionCommand::Header(command) => self.execute_header(command, session),
        };

        Ok(result)
    }

    fn execute_header(&self, command: HeaderCommand, session: &mut Session) -> ExecutionResult {
        match command {
            HeaderCommand::Set { name, value } => {
                session.headers().insert(name, value);
            }

            HeaderCommand::List => {
                let lines = session
                    .headers()
                    .iter()
                    .map(|(name, value)| format!("{name}: {value}"))
                    .collect::<Vec<_>>()
                    .join("\n");

                return ExecutionResult {
                    control_flow: ControlFlow::Continue,
                    output: Output::Text(lines),
                };
            }

            HeaderCommand::Remove { name } => {
                session.headers().remove(&name);
            }

            HeaderCommand::Clear => {
                session.headers().clear();
            }
        }

        ExecutionResult {
            control_flow: ControlFlow::Continue,
            output: Output::None,
        }
    }

    fn execute_variable(&self, command: VariableCommand, session: &mut Session) -> ExecutionResult {
        match command {
            VariableCommand::Set { name, value } => {
                session.variables().insert(name, value);
            }

            VariableCommand::List => {
                let lines = session
                    .variables()
                    .iter()
                    .map(|(name, value)| format!("{name} = {value}"))
                    .collect::<Vec<_>>()
                    .join("\n");

                return ExecutionResult {
                    control_flow: ControlFlow::Continue,
                    output: Output::Text(lines),
                };
            }

            VariableCommand::Remove { name } => {
                session.variables().remove(&name);
            }

            VariableCommand::Clear => {
                session.variables().clear();
            }
        }

        ExecutionResult {
            control_flow: ControlFlow::Continue,
            output: Output::None,
        }
    }

    fn execute_request(
        &mut self,
        command: RequestCommand,
        session: &mut Session,
    ) -> Result<ExecutionResult, ExecutionError> {
        let result = match command {
            RequestCommand::Save { name } => {
                session
                    .save_request(name.clone())
                    .map_err(ExecutionError::SessionError)?;

                ExecutionResult {
                    control_flow: ControlFlow::Continue,
                    output: Output::Text(format!("saved request: {name}")),
                }
            }

            RequestCommand::Run { name } => {
                let spec = session
                    .saved_requests()
                    .get(&name)
                    .cloned()
                    .ok_or_else(|| {
                        ExecutionError::SessionError(format!("no saved request: {name}"))
                    })?;

                return self.send_spec(spec, session);
            }

            RequestCommand::List => {
                let lines = session
                    .saved_requests()
                    .iter()
                    .map(|(name, spec)| {
                        format!("{} ({}) {}", name, spec.method.as_str(), spec.path)
                    })
                    .collect::<Vec<_>>()
                    .join("\n");

                ExecutionResult {
                    control_flow: ControlFlow::Continue,
                    output: Output::Text(lines),
                }
            }

            RequestCommand::Show { name } => {
                let spec = session
                    .saved_requests()
                    .get(&name)
                    .cloned()
                    .ok_or_else(|| {
                        ExecutionError::SessionError(format!("no saved request: {name}"))
                    })?;

                ExecutionResult {
                    control_flow: ControlFlow::Continue,
                    output: Output::Text(spec.to_string()),
                }
            }

            RequestCommand::Rename { old_name, new_name } => {
                session
                    .rename_request(&old_name, new_name.clone())
                    .map_err(ExecutionError::SessionError)?;

                ExecutionResult {
                    control_flow: ControlFlow::Continue,
                    output: Output::Text(format!("renamed {old_name} to {new_name}")),
                }
            }

            RequestCommand::Remove { name } => {
                session
                    .remove_request(&name)
                    .map_err(ExecutionError::SessionError)?;

                ExecutionResult {
                    control_flow: ControlFlow::Continue,
                    output: Output::Text(format!("removed request: {name}")),
                }
            }

            RequestCommand::Clear => {
                session.clear_requests();

                ExecutionResult {
                    control_flow: ControlFlow::Continue,
                    output: Output::Text("cleared saved requests".to_string()),
                }
            }
        };

        Ok(result)
    }

    fn send_spec(
        &mut self,
        spec: RequestSpec,
        session: &Session,
    ) -> Result<ExecutionResult, ExecutionError> {
        let request = session
            .build_request(spec)
            .map_err(ExecutionError::SessionError)?;

        let response = self
            .client
            .send(request)
            .map_err(|e| ExecutionError::HttpError(format!("request failed: {e}")))?;

        Ok(ExecutionResult {
            control_flow: ControlFlow::Continue,
            output: Output::HttpResponse(response),
        })
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
