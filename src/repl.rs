use std::io::Error;

use crate::reader::{ReadEvent, Reader};

pub struct Repl {
    reader: Reader,
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
        }
    }

    pub fn run(&mut self) -> Result<(), Error> {
        loop {
            match self.reader.read()? {
                ReadEvent::Input(input) => println!("You entered: {}", input),
                ReadEvent::Interrupt => println!("^C"),
                ReadEvent::Eof => break,
            }
        }

        Ok(())
    }
}
