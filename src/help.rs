use colored::Colorize;

pub fn help_text() -> String {
    format!(
        "
{}

{}

  {}:
    {} <path>
    [Headers]

    [Body]
    ::send

  {}:
    GET, POST, PUT, DELETE

  {}:
    <key>: <value>

  {}:
    raw, json

{}

  {}:
    base <url>
    header <key> <value>
    history
    rerun <id>
    help
    exit

{}
",
        "reqsh help".bold().cyan(),
        "─".repeat(50).dimmed(),
        "Requests".yellow().bold(),
        "Method".green().bold(),
        "Methods".yellow().bold(),
        "Headers".yellow().bold(),
        "Body".yellow().bold(),
        "─".repeat(50).dimmed(),
        "Commands".yellow().bold(),
        "─".repeat(50).dimmed(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn help_text_is_not_empty() {
        let text = help_text();

        assert!(!text.is_empty());
    }

    #[test]
    fn help_text_contains_requests_section() {
        let text = help_text();

        assert!(text.contains("Requests"));
    }

    #[test]
    fn help_text_contains_methods() {
        let text = help_text();

        assert!(text.contains("GET"));
        assert!(text.contains("POST"));
    }

    #[test]
    fn help_text_contains_commands() {
        let text = help_text();

        assert!(text.contains("help"));
        assert!(text.contains("exit"));
    }
}
