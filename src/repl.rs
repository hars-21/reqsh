use std::io::Error;
use std::time::Duration;

use crate::ast::{Command, HistoryCommand};
use crate::execute::{ControlFlow, Executor, Output};
use crate::lexer::Lexer;
use crate::parse::Parser;
use crate::printer::Printer;
use crate::reader::{ReadEvent, Reader};
use crate::session::Session;

pub struct Repl {
    reader: Reader,
    session: Session,
    executor: Executor,
}

impl Default for Repl {
    fn default() -> Self {
        Self::new()
    }
}

impl Repl {
    pub fn new() -> Self {
        Self {
            reader: Reader::new(),
            session: Session::load(),
            executor: Executor::new(),
        }
    }

    pub fn new_with_timeout(timeout: u64) -> Self {
        let mut repl = Self::new();
        repl.session.timeout().replace(Duration::from_secs(timeout));
        repl
    }

    pub fn run(&mut self) -> Result<(), Error> {
        loop {
            let input = match self.reader.read()? {
                ReadEvent::Input(input) => input,
                ReadEvent::Interrupt => continue,
                ReadEvent::Eof => break,
            };

            self.reader.save_history();

            if self.process(&input)? {
                break;
            }
        }

        if let Err(err) = self.session.save() {
            Printer::error(err);
        }

        Ok(())
    }

    fn process(&mut self, input: &str) -> Result<bool, Error> {
        let tokens = match Lexer::lex(input) {
            Ok(tokens) => tokens,
            Err(err) => {
                Printer::error(err);
                return Ok(false);
            }
        };

        let command = match Parser::parse(tokens) {
            Ok(command) => command,
            Err(err) => {
                Printer::error(err);
                return Ok(false);
            }
        };

        match command {
            Command::History(command) => self.execute_history(command),
            command => {
                let result = match self.executor.execute(command, &mut self.session) {
                    Ok(result) => result,
                    Err(err) => {
                        Printer::error(err);
                        return Ok(false);
                    }
                };

                match result.output {
                    Output::None => {}
                    Output::Text(text) => {
                        Printer::text(&text);
                    }
                    Output::HttpResponse(response) => {
                        Printer::http(&response);
                    }
                }

                match result.control_flow {
                    ControlFlow::Continue => Ok(false),
                    ControlFlow::Exit => {
                        Printer::text("Bye!");
                        Ok(true)
                    }
                }
            }
        }
    }

    fn execute_history(&mut self, command: HistoryCommand) -> Result<bool, Error> {
        match command {
            HistoryCommand::List => {
                for (index, line) in self.reader.history_lines().iter().enumerate() {
                    let indent = " ".repeat(6);
                    let mut lines = line.lines();

                    if let Some(first) = lines.next() {
                        Printer::text(format!("{:>4}: {}", index + 1, first));

                        for line in lines {
                            Printer::text(format!("{indent}{line}"));
                        }
                    }
                }
                Ok(false)
            }

            HistoryCommand::Show { index } => {
                match self.reader.history_line_at(index) {
                    Ok(line) => Printer::text(line.trim()),
                    Err(err) => Printer::error(err),
                }
                Ok(false)
            }

            HistoryCommand::Clear => {
                if let Err(err) = self.reader.history_clear() {
                    Printer::error(err);
                }
                Ok(false)
            }

            HistoryCommand::Rerun { index } => match self.reader.history_line_at(index) {
                Ok(line) => self.process(&line),
                Err(err) => {
                    Printer::error(err);
                    Ok(false)
                }
            },
        }
    }
}
