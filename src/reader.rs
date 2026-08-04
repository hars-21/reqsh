use std::{borrow::Cow, io::Error, path::PathBuf};

use reedline::{
    FileBackedHistory, HistoryItem, Prompt, PromptEditMode, PromptHistorySearch, Reedline,
    SearchDirection, SearchQuery, Signal, ValidationResult, Validator,
};

const REQUEST_METHODS: [&str; 7] = ["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"];

fn history_path() -> PathBuf {
    let home = dirs::home_dir().expect("could not determine home directory");
    home.join(".reqsh_history")
}

pub struct Reader {
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
            .unwrap_or_else(Reedline::create)
            .with_validator(Box::new(RequestValidator));

        Self {
            editor,
            prompt: ReqshPrompt { multiline: false },
        }
    }

    pub fn save_history(&mut self) {
        let _ = self.editor.sync_history();
    }

    pub fn read(&mut self) -> Result<ReadEvent, Error> {
        match self.editor.read_line(&self.prompt)? {
            Signal::Success(input) => Ok(ReadEvent::Input(input)),
            Signal::CtrlC => Ok(ReadEvent::Interrupt),
            Signal::CtrlD => Ok(ReadEvent::Eof),
            _ => unreachable!(),
        }
    }

    fn history_items(&self) -> impl Iterator<Item = HistoryItem> + '_ {
        self.editor
            .history()
            .search(SearchQuery::everything(SearchDirection::Forward, None))
            .unwrap_or_default()
            .into_iter()
    }

    pub fn history_lines(&self) -> Vec<String> {
        self.history_items().map(|item| item.command_line).collect()
    }

    pub fn history_line_at(&self, index: usize) -> Result<String, String> {
        if index == 0 {
            return Err("history indices start at 1".into());
        }

        self.history_items()
            .nth(index - 1)
            .map(|item| item.command_line)
            .ok_or_else(|| format!("history entry not found: {index}"))
    }

    pub fn history_clear(&mut self) -> Result<(), String> {
        self.editor
            .history_mut()
            .clear()
            .map_err(|e| e.to_string())?;
        self.save_history();
        Ok(())
    }
}

pub struct ReqshPrompt {
    pub multiline: bool,
}

impl Prompt for ReqshPrompt {
    fn render_prompt_left(&self) -> Cow<'_, str> {
        "reqsh> ".into()
    }

    fn render_prompt_right(&self) -> Cow<'_, str> {
        "".into()
    }

    fn render_prompt_indicator(&self, _: PromptEditMode) -> Cow<'_, str> {
        "".into()
    }

    fn render_prompt_multiline_indicator(&self) -> Cow<'_, str> {
        ".....>".into()
    }

    fn render_prompt_history_search_indicator(&self, _: PromptHistorySearch) -> Cow<'_, str> {
        "".into()
    }
}

pub struct RequestValidator;

impl Validator for RequestValidator {
    fn validate(&self, input: &str) -> ValidationResult {
        let first_line = input.lines().next().unwrap_or("");

        if !request_mode(first_line) {
            return ValidationResult::Complete;
        }

        let last_line = input.lines().last().unwrap_or("");

        if last_line.trim() == "###" {
            ValidationResult::Complete
        } else {
            ValidationResult::Incomplete
        }
    }
}

fn request_mode(line: &str) -> bool {
    line.split_whitespace().next().is_some_and(|token| {
        REQUEST_METHODS
            .iter()
            .any(|m| m.eq_ignore_ascii_case(token))
    })
}
