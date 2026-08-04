use std::{borrow::Cow, io::Error, mem, path::PathBuf};

use reedline::{FileBackedHistory, Prompt, PromptEditMode, PromptHistorySearch, Reedline, Signal};

const REQUEST_METHODS: [&str; 7] = ["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"];

fn history_path() -> PathBuf {
    let home = dirs::home_dir().expect("could not determine home directory");
    home.join(".reqsh_history")
}

pub struct Reader {
    input_buffer: String,
    editor: Reedline,
    prompt: ReqshPrompt,
}

#[derive(Debug)]
pub enum ReadEvent {
    Input(String),
    Interrupt,
    Eof,
}

impl Default for Reader {
    fn default() -> Self {
        Self::new()
    }
}

impl Reader {
    pub fn new() -> Self {
        let editor = FileBackedHistory::with_file(1000, history_path())
            .ok()
            .map(|history| Reedline::create().with_history(Box::new(history)))
            .unwrap_or_else(Reedline::create);

        Self {
            input_buffer: String::new(),
            editor,
            prompt: ReqshPrompt { multiline: false },
        }
    }

    pub fn save_history(&mut self) {
        let _ = self.editor.sync_history();
    }

    pub fn read(&mut self) -> Result<ReadEvent, Error> {
        let mut multiline = false;

        loop {
            let line = match self.editor.read_line(&self.prompt)? {
                Signal::Success(line) => line,
                Signal::CtrlC => {
                    self.prompt.multiline = false;
                    self.input_buffer.clear();
                    return Ok(ReadEvent::Interrupt);
                }
                Signal::CtrlD => {
                    self.prompt.multiline = false;
                    self.input_buffer.clear();
                    return Ok(ReadEvent::Eof);
                }
                _ => unreachable!(),
            };

            if multiline {
                if line.trim() == "###" {
                    break;
                }
                self.input_buffer.push_str(&line);
                self.input_buffer.push('\n');
            } else if line.trim().is_empty() {
                continue;
            } else {
                self.input_buffer.push_str(&line);
                self.input_buffer.push('\n');
                if !request_mode(&line) {
                    break;
                }
                multiline = true;
                self.prompt.multiline = true;
            }
        }

        self.prompt.multiline = false;
        Ok(ReadEvent::Input(mem::take(&mut self.input_buffer)))
    }
}

fn request_mode(line: &str) -> bool {
    line.split_whitespace().next().is_some_and(|token| {
        REQUEST_METHODS
            .iter()
            .any(|m| m.eq_ignore_ascii_case(token))
    })
}

pub struct ReqshPrompt {
    pub multiline: bool,
}

impl Prompt for ReqshPrompt {
    fn render_prompt_left(&self) -> Cow<'_, str> {
        if self.multiline {
            ".....> ".into()
        } else {
            "reqsh> ".into()
        }
    }

    fn render_prompt_right(&self) -> Cow<'_, str> {
        "".into()
    }

    fn render_prompt_indicator(&self, _: PromptEditMode) -> Cow<'_, str> {
        "".into()
    }

    fn render_prompt_multiline_indicator(&self) -> Cow<'_, str> {
        "".into()
    }

    fn render_prompt_history_search_indicator(&self, _: PromptHistorySearch) -> Cow<'_, str> {
        "".into()
    }
}
