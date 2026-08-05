use std::fmt;

use crate::{
    ast::{Command, RequestSpec},
    http::{Client, HttpResponse, describe_request_error},
    session::Session,
};

mod request;
mod session;
mod shell;
mod variable;

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
    HttpResponse {
        request_line: String,
        response: HttpResponse,
    },
}

pub struct ExecutionResult {
    pub control_flow: ControlFlow,
    pub output: Output,
}

#[derive(Debug)]
pub enum ExecutionError {
    InvalidRequest(String),
    RequestFailed(String),
    NoRequestToSave,
    SavedRequestNotFound(String),
    NoResponseToSave,
    NoRequestToReplay,
    WriteResponse { path: String, message: String },
}

impl fmt::Display for ExecutionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ExecutionError::InvalidRequest(message) => write!(f, "{message}"),
            ExecutionError::RequestFailed(message) => write!(f, "request failed: {message}"),
            ExecutionError::NoRequestToSave => {
                write!(
                    f,
                    "no request to save; run a request first, then use `req save <name>`"
                )
            }
            ExecutionError::SavedRequestNotFound(name) => {
                write!(f, "no saved request: {name}")
            }
            ExecutionError::NoResponseToSave => {
                write!(f, "no response to save; run a request first")
            }
            ExecutionError::NoRequestToReplay => {
                write!(f, "no request to replay; run a request first")
            }
            ExecutionError::WriteResponse { path, message } => {
                write!(f, "failed to write response to {path}: {message}")
            }
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

    fn send_spec(
        &mut self,
        spec: RequestSpec,
        session: &mut Session,
    ) -> Result<ExecutionResult, ExecutionError> {
        let request = session
            .build_request(spec)
            .map_err(ExecutionError::InvalidRequest)?;

        let request_line = format!("{} {}", request.method, request.url);

        let response = self
            .client
            .send(request)
            .map_err(|e| ExecutionError::RequestFailed(describe_request_error(&e)))?;

        *session.last_response() = Some(response.clone());

        Ok(ExecutionResult {
            control_flow: ControlFlow::Continue,
            output: Output::HttpResponse {
                request_line,
                response,
            },
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::{Command, Method, RequestSpec};

    #[test]
    fn connection_refused_is_described_gracefully() {
        let mut session = Session::new();
        let spec = RequestSpec {
            method: Method::Get,
            path: "http://127.0.0.1:1/".to_string(),
            headers: Vec::new(),
            queries: Vec::new(),
            body: String::new(),
        };

        let result = Executor::new().execute(Command::Http(spec), &mut session);

        assert!(matches!(
            result,
            Err(ExecutionError::RequestFailed(ref message))
                if message.contains("connection refused")
        ));
    }
}
