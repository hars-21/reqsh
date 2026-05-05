#[derive(Debug)]
pub enum Token {
    Word(String),
    String(String),
    Body(String),
    EOF,
}

enum Mode {
    Normal,
    String,
    Body,
}

pub fn tokenize(input: String) -> Vec<Token> {
    let mut tokens = Vec::new();
    let mut chars = input.chars().peekable();
    let mut mode = Mode::Normal;
    let mut value = String::new();

    while let Some(ch) = chars.next() {
        match ch {
            ' ' | '\t' | '\n' => match mode {
                Mode::Normal => {
                    tokens.push(Token::Word(value.clone()));
                    value.clear();
                }
                _ => {
                    value.push(ch);
                }
            },

            '\'' => match mode {
                Mode::Normal => {
                    mode = Mode::String;
                }
                Mode::String => {
                    tokens.push(Token::String(value.clone()));
                    value.clear();
                }
                Mode::Body => {
                    value.push(ch);
                }
            },

            '"' => match mode {
                Mode::Normal => {
                    mode = Mode::String;
                }
                Mode::String => {
                    tokens.push(Token::String(value.clone()));
                    value.clear();
                }
                Mode::Body => {
                    value.push(ch);
                }
            },

            '{' => match mode {
                Mode::Normal => {
                    mode = Mode::Body;
                    value.push(ch);
                }
                _ => {
                    value.push(ch);
                }
            },

            '}' => match mode {
                Mode::Body => {
                    value.push(ch);
                    tokens.push(Token::Body(value.clone()));
                    value.clear();
                }
                _ => {
                    value.push(ch);
                }
            },

            _ => value.push(ch),
        }
    }

    if !value.is_empty() {
        match mode {
            Mode::Normal => {
                tokens.push(Token::Word(value.clone()));
            }
            Mode::String => {
                tokens.push(Token::String(value.clone()));
            }
            Mode::Body => {
                tokens.push(Token::Body(value.clone()));
            }
        }
        value.clear();
    }

    tokens.push(Token::EOF);

    tokens
}
