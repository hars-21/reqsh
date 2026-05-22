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

    for (k, v) in res.headers() {
        let line = format!(
            "{}: {}\n",
            k.as_str().cyan().bold(),
            v.to_str().unwrap_or("")
        );

        output.push_str(&line);
    }

    output.push_str("\n");

    let body = res.text().unwrap_or_default();

    output.push_str(&body);
    output.push_str("\n");

    output
}
