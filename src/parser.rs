use crate::{
    builtin::Builtin,
    request::{Method, Request},
};

pub enum Parsed {
    Builtin(Builtin),
    Request(Request),
    Exit,
}

pub fn parse(input: String) -> Result<Parsed, String> {
    let first_line = input.lines().next().unwrap();
    let tokens: Vec<&str> = first_line.split_whitespace().collect();

    match tokens[0] {
        "GET" | "POST" | "PUT" | "DELETE" => {
            let result = parse_request(input)?;
            Ok(Parsed::Request(result))
        }

        "base" | "set" | "unset" | "header" | "headers" | "vars" | "save" | "run" | "help"
        | "history" | "rerun" => {
            let result = parse_builtin(input)?;
            Ok(Parsed::Builtin(result))
        }

        "exit" => Ok(Parsed::Exit),

        _ => Err(format!("Reference Error: {} not defined", { tokens[0] })),
    }
}

fn parse_request(buffer: String) -> Result<Request, String> {
    let lines: Vec<&str> = buffer.lines().collect();

    if lines.is_empty() {
        return Err("Empty request".to_string());
    }

    let req_parts: Vec<&str> = lines[0].split_whitespace().collect();

    if req_parts.len() != 2 {
        return Err("usage: METHOD <url>\n[headers]\n\n[body]\n::send".to_string());
    }

    let method = match req_parts[0].to_lowercase().as_str() {
        "get" => Method::GET,
        "post" => Method::POST,
        "put" => Method::PUT,
        "delete" => Method::DELETE,
        _ => return Err("Invalid Method".to_string()),
    };

    let path = req_parts[1].to_string();

    let mut request = Request::new(method, path);

    let mut body_lines = Vec::new();
    let mut reading_body = false;

    for line in lines.iter().skip(1) {
        if line.trim().is_empty() {
            reading_body = true;
            continue;
        }

        if reading_body {
            body_lines.push(*line);
        } else if let Some((key, value)) = line.split_once(':') {
            let key = key.trim();
            let value = value.trim();
            if key.eq_ignore_ascii_case("param") {
                if let Some((pk, pv)) = value.split_once('=') {
                    request.set_param(pk.trim().to_string(), pv.trim().to_string());
                } else {
                    return Err(format!(
                        "invalid param format: {value} (expected key=value)"
                    ));
                }
            } else {
                request.set_header(key.to_string(), value.to_string());
            }
        } else {
            return Err("Invalid headers".to_string());
        }
    }

    if !body_lines.is_empty() {
        request.set_body(body_lines.join("\n"));
    }

    Ok(request)
}

fn parse_builtin(line: String) -> Result<Builtin, String> {
    let tokens: Vec<&str> = line.split_whitespace().collect();
    match tokens[0] {
        "base" => {
            if tokens.len() != 2 {
                Err("usage: base <url>".to_string())
            } else {
                Ok(Builtin::Base(tokens[1].to_string()))
            }
        }
        "set" => {
            if tokens.len() != 3 {
                Err("usage: set <variable_name> <value>".to_string())
            } else {
                Ok(Builtin::Set(tokens[1].to_string(), tokens[2].to_string()))
            }
        }
        "unset" => {
            if tokens.len() == 2 {
                Ok(Builtin::UnsetVariable(tokens[1].to_string()))
            } else if tokens.len() == 3 && tokens[1] == "header" {
                Ok(Builtin::UnsetHeader(tokens[2].to_string()))
            } else {
                Err("usage: unset <name>\n       unset header <key>".to_string())
            }
        }
        "header" => {
            if tokens.len() != 3 {
                Err("usage: header <key> <value>".to_string())
            } else {
                Ok(Builtin::Header(
                    tokens[1].to_string(),
                    tokens[2].to_string(),
                ))
            }
        }
        "headers" => {
            if tokens.len() != 1 {
                Err("usage: headers".to_string())
            } else {
                Ok(Builtin::Headers)
            }
        }
        "vars" => {
            if tokens.len() != 1 {
                Err("usage: vars".to_string())
            } else {
                Ok(Builtin::Vars)
            }
        }
        "save" => {
            if tokens.len() != 2 {
                Err("usage: save <name>".to_string())
            } else {
                Ok(Builtin::Save(tokens[1].to_string()))
            }
        }
        "run" => {
            if tokens.len() != 2 {
                Err("usage: run <name>".to_string())
            } else {
                Ok(Builtin::Run(tokens[1].to_string()))
            }
        }
        "help" => Ok(Builtin::Help),
        "history" => Ok(Builtin::History),
        "rerun" => {
            if tokens.len() != 2 {
                Err("usage: rerun <index>".to_string())
            } else {
                let idx = tokens
                    .get(1)
                    .ok_or("missing index")?
                    .parse::<usize>()
                    .map_err(|e| format!("invalid index: {}", e))?;
                Ok(Builtin::Rerun(idx))
            }
        }
        _ => Err("Invalid Command".to_string()),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_get_request() {
        let input = "GET /users".to_string();
        let result = parse(input);

        assert!(result.is_ok());

        match result.unwrap() {
            Parsed::Request(req) => {
                assert_eq!(req.method, Method::GET);
                assert_eq!(req.path, "/users");
            }

            _ => panic!("expected request"),
        }
    }

    #[test]
    fn parse_help_builtin() {
        let input = "help".to_string();
        let result = parse(input);

        assert!(result.is_ok());

        match result.unwrap() {
            Parsed::Builtin(Builtin::Help) => {}

            _ => panic!("expected help builtin"),
        }
    }

    #[test]
    fn parse_unknown_command_returns_error() {
        let input = "something".to_string();
        let result = parse(input);

        assert!(result.is_err());
    }
}
