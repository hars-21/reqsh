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
