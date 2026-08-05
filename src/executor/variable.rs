use super::{ControlFlow, ExecutionResult, Executor, Output};
use crate::ast::VariableCommand;
use crate::session::Session;

impl Executor {
    pub(super) fn execute_variable(
        &self,
        command: VariableCommand,
        session: &mut Session,
    ) -> ExecutionResult {
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

                let message = if lines.is_empty() {
                    "no variables set".to_string()
                } else {
                    lines
                };

                return ExecutionResult {
                    control_flow: ControlFlow::Continue,
                    output: Output::Text(message),
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
}
