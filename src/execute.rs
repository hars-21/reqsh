use crate::{
    ast::{Command, ShellCommand},
    help::help_text,
    session::Session,
};

pub struct Executor;

pub enum ControlFlow {
    Continue,
    Exit,
}

pub enum Output {
    Text(String),
    None,
}

pub struct ExecutionResult {
    pub control_flow: ControlFlow,
    pub output: Output,
}

impl Executor {
    pub fn new() -> Self {
        Self
    }

    pub fn execute(&mut self, command: Command, session: &mut Session) -> ExecutionResult {
        match command {
            Command::Session(command) => {
                session.apply_session(command);
                ExecutionResult {
                    control_flow: ControlFlow::Continue,
                    output: Output::None,
                }
            }

            Command::Variable(command) => {
                session.apply_variable(command);
                ExecutionResult {
                    control_flow: ControlFlow::Continue,
                    output: Output::None,
                }
            }

            Command::Shell(command) => self.execute_shell(command),

            Command::History(_) => {
                todo!("history")
            }

            Command::Request(_) => {
                todo!("saved requests")
            }

            Command::Http(_) => {
                todo!("http requests")
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
