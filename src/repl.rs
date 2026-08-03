use std::io::Error;

use crate::execute::{ControlFlow, Executor, Output};
use crate::parse::parse;
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
            session: Session::new(),
            executor: Executor::new(),
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        loop {
            let input = match self.reader.read()? {
                ReadEvent::Input(input) => input,
                ReadEvent::Interrupt => continue,
                ReadEvent::Eof => break,
            };

            let command = match parse(&input) {
                Ok(command) => command,
                Err(e) => {
                    eprintln!("{}", e);
                    continue;
                }
            };

            let result = self.executor.execute(command, &mut self.session);

            match result.output {
                Output::Text(text) => {
                    print!("{}", text);
                }
                Output::None => {}
            }

            match result.control_flow {
                ControlFlow::Exit => break,
                ControlFlow::Continue => {}
            }
        }

        Ok(())
    }
}
