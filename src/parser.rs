use std::collections::HashMap;

use crate::request::{Method, Request};

const METHODS: &[&str] = &["GET", "POST", "PUT", "DELETE"];
const COMMANDS: &[&str] = &["base", "header", "help"];

pub enum InputType {
    Request,
    Command,
    Error(String),
    Exit,
}

impl InputType {
    pub fn get(line: &str) -> Self {
        let tokens: Vec<&str> = line.split_whitespace().collect();
        let first_token = tokens[0];
        if first_token == "exit" {
            InputType::Exit
        } else if METHODS.contains(&first_token) {
            InputType::Request
        } else if COMMANDS.contains(&first_token) {
            InputType::Command
        } else {
            InputType::Error(format!("Reference Error: {} not defined", first_token))
        }
    }
}

pub fn parse_request(buffer: String) -> Result<Request, String> {
    if let Some((header_part, body_part)) = buffer.split_once("\n\n") {
        let header_lines: Vec<&str> = header_part.split('\n').collect();

        let req_parts: Vec<&str> = header_lines[0].split_whitespace().collect();
        if req_parts.len() != 2 {
            return Err(format!("usage: METHOD <url> \n[headers]\n[body]"));
        }

        let method = match req_parts[0].to_lowercase().as_str() {
            "get" => Method::GET,
            "post" => Method::POST,
            "put" => Method::PUT,
            "delete" => Method::DELETE,
            _ => panic!("Invalid Method"),
        };
        let path = req_parts[1];

        let mut headers = HashMap::new();
        if header_lines.len() > 1 {
            for line in header_lines.iter().skip(1) {
                if let Some((key, value)) = line.split_once(':') {
                    headers.insert(key.trim().to_string(), value.trim().to_string());
                } else {
                    return Err(format!("Invalid headers"));
                }
            }
        }

        let body = if body_part.trim().is_empty() {
            None
        } else {
            Some(body_part.trim().to_string())
        };

        println!(
            "{:?} \n{:?} \n{:?} \n{:?} \n{:?} \n{:?}",
            header_part, header_lines, req_parts, headers, body_part, body
        );

        Ok(Request {
            method,
            path: path.to_string(),
            headers,
            body,
        })
    } else {
        Err(format!("usage: METHOD <url> \n[headers]\n[body]"))
    }
}

pub enum Command {
    Base(String),
    Header(String, String),
    Help,
}

pub fn parse_command(line: &str) -> Result<Command, String> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    match tokens[0] {
        "base" => {
            if tokens.len() != 2 {
                Err(format!("usage: base <url>"))
            } else {
                Ok(Command::Base(tokens[1].to_string()))
            }
        }
        "header" => {
            if tokens.len() != 3 {
                Err(format!("usage: header <key> <value>"))
            } else {
                Ok(Command::Header(
                    tokens[1].to_string(),
                    tokens[2].to_string(),
                ))
            }
        }
        "help" => Ok(Command::Help),
        _ => Err(format!("Invalid Command")),
    }
}
