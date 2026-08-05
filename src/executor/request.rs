use super::{ControlFlow, ExecutionError, ExecutionResult, Executor, Output};
use crate::ast::RequestCommand;
use crate::session::Session;

impl Executor {
    pub(super) fn execute_request(
        &mut self,
        command: RequestCommand,
        session: &mut Session,
    ) -> Result<ExecutionResult, ExecutionError> {
        let result = match command {
            RequestCommand::Save { name } => {
                session
                    .save_request(name.clone())
                    .map_err(|_| ExecutionError::NoRequestToSave)?;

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
                    .ok_or_else(|| ExecutionError::SavedRequestNotFound(name.clone()))?;

                *session.last_request() = Some(spec.clone());

                return self.send_spec(spec, session);
            }

            RequestCommand::Rerun => {
                let spec = session
                    .last_request()
                    .clone()
                    .ok_or(ExecutionError::NoRequestToReplay)?;

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

                let message = if lines.is_empty() {
                    "no saved requests".to_string()
                } else {
                    lines
                };

                ExecutionResult {
                    control_flow: ControlFlow::Continue,
                    output: Output::Text(message),
                }
            }

            RequestCommand::Show { name } => {
                let spec = session
                    .saved_requests()
                    .get(&name)
                    .cloned()
                    .ok_or_else(|| ExecutionError::SavedRequestNotFound(name.clone()))?;

                ExecutionResult {
                    control_flow: ControlFlow::Continue,
                    output: Output::Text(spec.to_string()),
                }
            }

            RequestCommand::Rename { old_name, new_name } => {
                session
                    .rename_request(&old_name, new_name.clone())
                    .map_err(|_| ExecutionError::SavedRequestNotFound(old_name.clone()))?;

                ExecutionResult {
                    control_flow: ControlFlow::Continue,
                    output: Output::Text(format!("renamed {old_name} to {new_name}")),
                }
            }

            RequestCommand::Remove { name } => {
                session
                    .remove_request(&name)
                    .map_err(|_| ExecutionError::SavedRequestNotFound(name.clone()))?;

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

            RequestCommand::SaveResponse { path } => {
                let body = session
                    .last_response()
                    .as_ref()
                    .map(|response| response.body.clone())
                    .ok_or(ExecutionError::NoResponseToSave)?;

                std::fs::write(&path, body).map_err(|e| ExecutionError::WriteResponse {
                    path: path.clone(),
                    message: e.to_string(),
                })?;

                ExecutionResult {
                    control_flow: ControlFlow::Continue,
                    output: Output::Text(format!("saved response to {path}")),
                }
            }
        };

        Ok(result)
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use crate::ast::{Command, RequestCommand};
    use crate::executor::{ExecutionError, Executor, Output};
    use crate::http::HttpResponse;
    use crate::session::Session;

    fn response(body: &str) -> HttpResponse {
        HttpResponse {
            version: "HTTP/1.1".to_string(),
            status: 200,
            reason: "OK".to_string(),
            headers: Vec::new(),
            body: body.to_string(),
            duration: Duration::from_secs(0),
        }
    }

    #[test]
    fn save_response_writes_body_to_file() {
        let mut session = Session::new();
        session.last_response().replace(response("hello world"));

        let path = std::env::temp_dir().join("reqsh_test_response.txt");
        let result = Executor::new()
            .execute(
                Command::Request(RequestCommand::SaveResponse {
                    path: path.to_string_lossy().to_string(),
                }),
                &mut session,
            )
            .unwrap();

        let Output::Text(message) = result.output else {
            panic!("expected text output");
        };
        assert!(message.contains("saved response"));

        assert_eq!(std::fs::read_to_string(&path).unwrap(), "hello world");
        let _ = std::fs::remove_file(path);
    }

    #[test]
    fn save_response_without_request_errors() {
        let mut session = Session::new();

        let result = Executor::new().execute(
            Command::Request(RequestCommand::SaveResponse {
                path: "never_written.json".to_string(),
            }),
            &mut session,
        );

        assert!(matches!(result, Err(ExecutionError::NoResponseToSave)));
    }

    #[test]
    fn rerun_without_request_errors() {
        let mut session = Session::new();

        let result = Executor::new().execute(Command::Request(RequestCommand::Rerun), &mut session);

        assert!(matches!(result, Err(ExecutionError::NoRequestToReplay)));
    }
}
