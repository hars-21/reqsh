use colored::Colorize;

pub fn help_text() -> String {
    format!(
        "
{}
{}
  {}:
    reqsh [options]
  {}:
    {}           Show help
    {}        Show version
    {}  Set request timeout
{}
  {}:
    {} <path>        start of a request
    <name>=<value>       query parameter
    <name>: <value>      request header
    <blank line>         body starts after an empty line
    <body>
    ###                  end of request
  {}:
    GET  POST  PUT  PATCH  DELETE  HEAD  OPTIONS
  {}:
    GET /users?page=1
    Accept: application/json
    Authorization: Bearer <token>
    ###
{}
  {}:
    base <url>
    header set|list|remove|clear <key> [value]
    var set|list|remove|clear <name> [value]
    req (request) save|run|list|show|remove|rename|clear <name>
    history [list]|show|clear|rerun <id>
    timeout <seconds>
    clear
    version
    help
    exit
{}
",
        "reqsh - Interactive HTTP Shell".bold().cyan(),
        "─".repeat(50).dimmed(),
        "Usage".yellow().bold(),
        "Options".yellow().bold(),
        "--help, -h".green().bold(),
        "--version, -v".green().bold(),
        "--timeout <seconds>".green().bold(),
        "─".repeat(50).dimmed(),
        "Requests".yellow().bold(),
        "METHOD".green().bold(),
        "Methods".yellow().bold(),
        "Example".yellow().bold(),
        "─".repeat(50).dimmed(),
        "Commands".yellow().bold(),
        "─".repeat(50).dimmed(),
    )
}
