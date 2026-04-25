use crate::context::RequestContext;
use crate::help;
use crate::http::fetch;
use crate::parser::ShellCommand;
use crate::request::{Method, Request};

pub struct ShellOutput {
    pub signal: ShellSignal,
    pub output: Option<String>,
}

pub enum ShellSignal {
    Continue,
    Exit,
}

pub fn execute(cmd: &ShellCommand, ctx: &mut RequestContext) -> ShellOutput {
    let mut output;

    match cmd.name.as_str() {
        "exit" => {
            return ShellOutput {
                signal: ShellSignal::Exit,
                output: None,
            };
        }

        "help" => {
            output = format!("{}", help::get_help());
        }

        "set" => {
            if cmd.args.len() == 2 && cmd.args[0] == "base_url" {
                ctx.set_base_url(&cmd.args[1]);
                output = format!("Base URL set to: {}", cmd.args[1]);
            } else {
                output = format!("Usage: set base_url <url>");
            }
        }

        "save" => {
            if cmd.args.len() == 3 {
                output = format!("Saved request '{}'", cmd.args[0]);
                if cmd.args[1] == "GET" {
                    ctx.save_request(&cmd.args[0], Method::GET, cmd.args[2].clone());
                } else if cmd.args[1] == "POST" {
                    ctx.save_request(&cmd.args[0], Method::GET, cmd.args[2].clone());
                } else {
                    output = format!("Incorrect method");
                }
            } else {
                output = format!("Usage: save <request_name> <method> <url>");
            }
        }

        "run" => {
            if cmd.args.len() == 1 {
                if let Some(request) = ctx.get_saved_request(&cmd.args[0]) {
                    let response = fetch(request, ctx.get_base_url());
                    output = format!("{}", response);
                } else {
                    output = format!("No saved request found with name '{}'", cmd.args[0]);
                }
            } else {
                output = format!("Usage: run <request_name>");
            }
        }

        "list" => {
            output = format!("Saved requests:\n");
            for name in ctx.list_saved_requests() {
                output.push_str(&format!("  {}\n", name));
            }
        }

        "delete" => {
            if cmd.args.len() == 1 {
                if ctx.delete_saved_request(&cmd.args[0]) {
                    output = format!("Deleted saved request '{}'", cmd.args[0]);
                } else {
                    output = format!("No saved request found with name '{}'", cmd.args[0]);
                }
            } else {
                output = format!("Usage: delete <request_name>");
            }
        }

        "headers" => {
            if cmd.args.len() == 1 {
                if let Some(request) = ctx.get_saved_request(&cmd.args[0]) {
                    output = format!("Headers for '{}':\n", cmd.args[0]);
                    for (key, value) in &request.headers {
                        output.push_str(&format!("  {}: {}\n", key, value));
                    }
                } else {
                    output = format!("No saved request found with name '{}'", cmd.args[0]);
                }
            } else if cmd.args.len() == 4 && cmd.args[0] == "set" {
                if let Some(request) = ctx.get_saved_request_mut(&cmd.args[1]) {
                    request.set_header(cmd.args[2].clone().to_lowercase(), cmd.args[3].clone());
                    output = format!(
                        "Set header '{}' to '{}' for request '{}'",
                        cmd.args[2], cmd.args[3], cmd.args[1]
                    );
                } else {
                    output = format!("No saved request found with name '{}'", cmd.args[1]);
                }
            } else if cmd.args.len() == 2 && cmd.args[0] == "clear" {
                if let Some(request) = ctx.get_saved_request_mut(&cmd.args[1]) {
                    request.headers.clear();
                    output = format!("Cleared headers for request '{}'", cmd.args[1]);
                } else {
                    output = format!("No saved request found with name '{}'", cmd.args[1]);
                }
            } else {
                output = format!("Usage: headers <request_name>");
            }
        }

        "GET" => {
            if cmd.args.len() == 1 {
                let request = Request::new(Method::GET, cmd.args[0].clone());
                let response = fetch(&request, ctx.get_base_url());
                output = format!("{}", response);
            } else {
                output = format!("Usage: GET <url>");
            }
        }

        "POST" => {
            if cmd.args.len() >= 1 {
                let mut request = Request::new(Method::POST, cmd.args[0].clone());
                if let Some(body) = cmd.args.get(2) {
                    request.set_body(body);
                }
                let response = fetch(&request, ctx.get_base_url());
                output = format!("{}", response);
            } else {
                output = format!("Usage: POST <url> <body>");
            }
        }

        _ => {
            output = format!("ReferenceError: {} is not defined", cmd.name);
        }
    }

    ShellOutput {
        signal: ShellSignal::Continue,
        output: Some(output),
    }
}
