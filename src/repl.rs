use std::io::Error;

use crate::execute::{ControlFlow, Executor};
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

            println!("Parsed command: {:?}", command);

            match self.executor.execute(command, &mut self.session) {
                Ok(result) => {
                    println!("{:?}", result.output);

                    if matches!(result.control_flow, ControlFlow::Exit) {
                        break;
                    }
                }

                Err(err) => {
                    eprintln!("Error: {:?}", err);
                }
            }
        }

        Ok(())
    }
}
