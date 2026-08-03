use std::{collections::HashMap, time::Duration};

use crate::ast::{HeaderCommand, SessionCommand, VariableCommand};

#[derive(Debug, Default)]
pub struct Session {
    base_url: Option<String>,
    headers: HashMap<String, String>,
    variables: HashMap<String, String>,
    timeout: Option<Duration>,
}

impl Session {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn apply_session(&mut self, command: SessionCommand) {
        match command {
            SessionCommand::Base(url) => {
                self.base_url = Some(url);
            }

            SessionCommand::Timeout(timeout) => {
                self.timeout = Some(timeout);
            }

            SessionCommand::Header(command) => {
                self.apply_header(command);
            }
        }
    }

    pub fn apply_variable(&mut self, command: VariableCommand) {
        match command {
            VariableCommand::Set { name, value } => {
                self.variables.insert(name, value);
            }

            VariableCommand::Remove { name } => {
                self.variables.remove(&name);
            }

            VariableCommand::Clear => {
                self.variables.clear();
            }

            VariableCommand::List => {}
        }
    }

    fn apply_header(&mut self, command: HeaderCommand) {
        match command {
            HeaderCommand::Set { name, value } => {
                self.headers.insert(name, value);
            }

            HeaderCommand::Remove { name } => {
                self.headers.remove(&name);
            }

            HeaderCommand::Clear => {
                self.headers.clear();
            }

            HeaderCommand::List => {}
        }
    }
}
