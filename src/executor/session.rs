use reqwest::Url;

use super::{ControlFlow, ExecutionError, ExecutionResult, Executor, Output};
use crate::ast::{HeaderCommand, SessionCommand};
use crate::session::Session;

impl Executor {
    pub(super) fn execute_session(
        &self,
        command: SessionCommand,
        session: &mut Session,
    ) -> Result<ExecutionResult, ExecutionError> {
        let result = match command {
            SessionCommand::Base(url) => {
                Url::parse(&url).map_err(|e| {
                    ExecutionError::InvalidRequest(format!("invalid base URL `{url}`: {e}"))
                })?;

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

    pub(super) fn execute_header(
        &self,
        command: HeaderCommand,
        session: &mut Session,
    ) -> ExecutionResult {
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

                let message = if lines.is_empty() {
                    "no headers set".to_string()
                } else {
                    lines
                };

                return ExecutionResult {
                    control_flow: ControlFlow::Continue,
                    output: Output::Text(message),
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
}
