use colored::Colorize;
use reqwest::blocking::Response;

pub fn display_response(res: Response) -> String {
    let mut output = String::new();

    let status = res.status();
    let reason = status.canonical_reason().unwrap_or("");

    let (status_color, reason_color) = if status.is_success() {
        (status.as_str().green(), reason.green())
    } else if status.is_client_error() {
        (status.as_str().yellow(), reason.yellow())
    } else if status.is_server_error() {
        (status.as_str().red(), reason.red())
    } else {
        (status.as_str().normal(), reason.normal())
    };

    let status_line = format!(
        "{} {} {}\n",
        format!("{:?}", res.version()).magenta(),
        status_color.bold(),
        reason_color.bold()
    );

    output.push_str(&status_line);

    let mut is_json_body = false;

    for (key, value) in res.headers() {
        let line = format!(
            "{}: {}\n",
            key.as_str().cyan().bold(),
            value.to_str().unwrap_or("")
        );

        if key.as_str().to_lowercase() == "content-type"
            && value.to_str().unwrap_or("").contains("application/json")
        {
            is_json_body = true;
        }

        output.push_str(&line);
    }

    output.push('\n');

    let raw = res.text().unwrap_or_default();
    let body;

    if is_json_body {
        match serde_json::from_str::<serde_json::Value>(&raw) {
            Ok(json) => {
                body = serde_json::to_string_pretty(&json).unwrap_or(raw);
            }
            Err(_) => {
                body = raw;
            }
        }
    } else {
        body = raw;
    }

    output.push_str(&body);
    output.push('\n');

    output
}
