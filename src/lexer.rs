use std::fmt;

#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    Word(String),
    String(String),
    Variable(String),
    Newline,
    Body(String),
    End,
}

pub enum LexerError {
    UnterminatedString,
    InvalidVariable,
}

impl fmt::Display for LexerError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LexerError::UnterminatedString => {
                write!(f, "unterminated string: missing closing quote")
            }

            LexerError::InvalidVariable => {
                write!(f, "invalid variable: expected format {{name}}")
            }
        }
    }
}

pub struct Lexer;

impl Lexer {
    pub fn lex(input: &str) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

        let mut lines = input.lines().peekable();

        while let Some(line) = lines.next() {
            // Empty line means body starts
            if line.trim().is_empty() {
                let body = lines.collect::<Vec<_>>().join("\n");

                if !body.is_empty() {
                    tokens.push(Token::Body(body));
                }

                break;
            }

            tokens.extend(Self::lex_line(line)?);
            tokens.push(Token::Newline);
        }

        tokens.push(Token::End);

        Ok(tokens)
    }

    fn lex_line(line: &str) -> Result<Vec<Token>, LexerError> {
        let mut tokens = Vec::new();

        let chars: Vec<char> = line.chars().collect();
        let mut current = String::new();

        let mut i = 0;

        while i < chars.len() {
            match chars[i] {
                // Quoted string
                '"' | '\'' => {
                    if !current.is_empty() {
                        tokens.push(Token::Word(current.clone()));
                        current.clear();
                    }

                    let quote = chars[i];
                    i += 1;

                    let mut value = String::new();

                    while i < chars.len() {
                        let c = chars[i];

                        // Closing quote
                        if c == quote {
                            break;
                        }

                        // Escape handling
                        if c == '\\' {
                            i += 1;

                            if i >= chars.len() {
                                return Err(LexerError::UnterminatedString);
                            }

                            let escaped = match chars[i] {
                                'n' => '\n',
                                't' => '\t',
                                '\\' => '\\',
                                '"' => '"',
                                '\'' => '\'',
                                other => other,
                            };

                            value.push(escaped);
                            i += 1;
                            continue;
                        }

                        value.push(c);
                        i += 1;
                    }

                    if i >= chars.len() {
                        return Err(LexerError::UnterminatedString);
                    }

                    tokens.push(Token::String(value));

                    i += 1;
                }

                // Variable {{name}}
                '{' if i + 1 < chars.len() && chars[i + 1] == '{' => {
                    if !current.is_empty() {
                        tokens.push(Token::Word(current.clone()));
                        current.clear();
                    }

                    i += 2;

                    let mut name = String::new();

                    while i + 1 < chars.len() && !(chars[i] == '}' && chars[i + 1] == '}') {
                        name.push(chars[i]);
                        i += 1;
                    }

                    if i + 1 >= chars.len() {
                        return Err(LexerError::InvalidVariable);
                    }

                    let name = name.trim();

                    // Validate variable name
                    if name.is_empty()
                        || !name.chars().all(|c| c.is_ascii_alphanumeric() || c == '_')
                    {
                        return Err(LexerError::InvalidVariable);
                    }

                    tokens.push(Token::Variable(name.to_string()));

                    i += 2;
                }

                // Whitespace ends word
                ' ' | '\t' => {
                    if !current.is_empty() {
                        tokens.push(Token::Word(current.clone()));
                        current.clear();
                    }

                    i += 1;
                }

                // Everything else is part of word
                c => {
                    current.push(c);
                    i += 1;
                }
            }
        }

        if !current.is_empty() {
            tokens.push(Token::Word(current));
        }

        Ok(tokens)
    }
}
