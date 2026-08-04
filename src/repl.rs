use std::io::Error;

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
        repl.session.set_timeout(timeout);
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

            let tokens = match Lexer::lex(&input) {
                Ok(tokens) => tokens,
                Err(err) => {
                    Printer::error(err);
                    continue;
                }
            };

            let command = match Parser::parse(tokens) {
                Ok(command) => command,
                Err(err) => {
                    Printer::error(err);
                    continue;
                }
            };

            let result = match self.executor.execute(command, &mut self.session) {
                Ok(result) => result,
                Err(err) => {
                    Printer::error(err);
                    continue;
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
                ControlFlow::Continue => {}
                ControlFlow::Exit => {
                    Printer::text("Bye!");
                    break;
                }
            }
        }

        if let Err(err) = self.session.save() {
            Printer::error(err);
        }

        Ok(())
    }
}
