use std::iter::Peekable;
use std::str::Chars;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Token {
    Word(String),
    String(String),
    Variable(String),
    Colon,
    Equals,
    Newline,
    End,
}

#[derive(Debug)]
pub enum LexerError {
    UnterminatedString,
    InvalidVariable,
}

pub struct Lexer<'a> {
    chars: Peekable<Chars<'a>>,
}

impl<'a> Lexer<'a> {
    pub fn new(input: &'a str) -> Self {
        Self {
            chars: input.chars().peekable(),
        }
    }

    pub fn lex(input: &'a str) -> Result<Vec<Token>, LexerError> {
        let mut lexer = Lexer::new(input);
        let mut tokens = Vec::new();

        while let Some(token) = lexer.next()? {
            tokens.push(token);
        }

        Ok(tokens)
    }

    fn next(&mut self) -> Result<Option<Token>, LexerError> {
        loop {
            let ch = match self.chars.peek() {
                Some(ch) => *ch,
                None => return Ok(None),
            };

            match ch {
                ' ' | '\t' | '\r' => {
                    self.chars.next();
                }

                '\n' => {
                    self.chars.next();
                    return Ok(Some(Token::Newline));
                }

                ':' => {
                    self.chars.next();
                    return Ok(Some(Token::Colon));
                }

                '=' => {
                    self.chars.next();
                    return Ok(Some(Token::Equals));
                }

                '"' => {
                    return self.read_string();
                }

                '#' => {
                    if self.read_end() {
                        return Ok(Some(Token::End));
                    }

                    self.skip_comment();
                }

                '{' => {
                    if self.peek_variable() {
                        return self.read_variable();
                    }

                    return Ok(Some(self.read_word()));
                }

                _ => {
                    return Ok(Some(self.read_word()));
                }
            }
        }
    }

    fn read_word(&mut self) -> Token {
        let mut word = String::new();

        while let Some(&ch) = self.chars.peek() {
            if is_delimiter(ch) {
                break;
            }

            word.push(ch);
            self.chars.next();
        }

        Token::Word(word)
    }

    fn read_string(&mut self) -> Result<Option<Token>, LexerError> {
        self.chars.next();

        let mut value = String::new();

        while let Some(ch) = self.chars.next() {
            if ch == '"' {
                return Ok(Some(Token::String(value)));
            }

            value.push(ch);
        }

        Err(LexerError::UnterminatedString)
    }

    fn read_end(&mut self) -> bool {
        let mut clone = self.chars.clone();

        if clone.next() != Some('#') {
            return false;
        }

        if clone.next() != Some('#') {
            return false;
        }

        if clone.next() != Some('#') {
            return false;
        }

        self.chars.next();
        self.chars.next();
        self.chars.next();

        true
    }

    fn skip_comment(&mut self) {
        while let Some(ch) = self.chars.next() {
            if ch == '\n' {
                break;
            }
        }
    }

    fn peek_variable(&self) -> bool {
        let mut clone = self.chars.clone();

        clone.next() == Some('{') && clone.next() == Some('{')
    }

    fn read_variable(&mut self) -> Result<Option<Token>, LexerError> {
        self.chars.next();
        self.chars.next();

        while matches!(self.chars.peek(), Some(' ' | '\t')) {
            self.chars.next();
        }

        let mut name = String::new();

        while let Some(&ch) = self.chars.peek() {
            if ch == '}' {
                break;
            }

            name.push(ch);
            self.chars.next();
        }

        let name = name.trim().to_string();

        if self.chars.next() != Some('}') {
            return Err(LexerError::InvalidVariable);
        }

        if self.chars.next() != Some('}') {
            return Err(LexerError::InvalidVariable);
        }

        Ok(Some(Token::Variable(name)))
    }
}

fn is_delimiter(ch: char) -> bool {
    matches!(ch, ' ' | '\t' | '\r' | '\n' | ':' | '=' | '"' | '#' | '{')
}
