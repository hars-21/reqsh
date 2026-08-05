use super::{ControlFlow, ExecutionResult, Executor, Output};
use crate::ast::ShellCommand;
use crate::help::help_text;

impl Executor {
    pub(super) fn execute_shell(&self, command: ShellCommand) -> ExecutionResult {
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
