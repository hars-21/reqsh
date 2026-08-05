use std::fmt::Display;

use colored::Colorize;
use serde_json::Value;

use crate::http::HttpResponse;

pub struct Printer;

impl Printer {
    /// Print plain text output.
    pub fn text(message: impl AsRef<str>) {
        println!("{}", message.as_ref());
    }

    /// Print an error.
    pub fn error(error: impl Display) {
        eprintln!("{}", format!("error: {}", error).red().bold());
    }

    /// Print an HTTP response.
    pub fn http(request_line: &str, response: &HttpResponse) {
        println!("{}", request_line.bold().cyan());
        println!();
        println!("{}", Self::format_http(response));
    }

    fn format_http(response: &HttpResponse) -> String {
        let mut output = String::new();

        let status = response.status;

        let status_text = match status {
            200..=299 => status.to_string().green().bold(),
            400..=499 => status.to_string().yellow().bold(),
            500..=599 => status.to_string().red().bold(),
            _ => status.to_string().normal(),
        };

        output.push_str(&format!(
            "{} {} {} {}\n",
            response.version.magenta(),
            status_text,
            response.reason.normal(),
            format!("{}ms", response.duration.as_millis()).bright_yellow(),
        ));

        let mut is_json = false;

        for header in &response.headers {
            output.push_str(&format!(
                "{}: {}\n",
                header.name.cyan().bold(),
                header.value
            ));

            if header.name.eq_ignore_ascii_case("content-type")
                && header.value.contains("application/json")
            {
                is_json = true;
            }
        }

        output.push('\n');

        if !response.body.is_empty() {
            if is_json {
                match serde_json::from_str::<Value>(&response.body) {
                    Ok(json) => match serde_json::to_string_pretty(&json) {
                        Ok(pretty) => output.push_str(&pretty),
                        Err(_) => output.push_str(&response.body),
                    },
                    Err(_) => output.push_str(&response.body),
                }
            } else {
                output.push_str(&response.body);
            }

            output.push('\n');
        }

        output
    }
}
