pub struct Repl {}

impl Repl {
    pub fn new() -> Self {
        Self {}
    }

    pub fn run(&mut self) {
        let mut rl = rustyline::DefaultEditor::new().unwrap();

        loop {
            let readline = rl.readline("reqsh> ");
            match readline {
                Ok(line) => {
                    if line.trim().is_empty() {
                        continue;
                    }
                }

                Err(err) => {
                    eprintln!("Error: {:?}", err);
                    break;
                }
            }
        }
    }
}
