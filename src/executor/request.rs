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
        };

        Ok(result)
    }
}
