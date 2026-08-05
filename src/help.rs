use colored::Colorize;

const RULE: &str = "─";

pub fn help_text() -> String {
    let mut help = String::new();

    help.push_str(&format!("{}\n", "reqsh - Interactive HTTP Shell".bold().cyan()));
    help.push_str(&rule());

    help.push_str(&format!("{}\n", "Usage:".yellow().bold()));
    help.push_str(&format!("  {}\n", "reqsh [options]".green().bold()));

    help.push('\n');

    help.push_str(&format!("{}\n", "Options:".yellow().bold()));
    help.push_str(&line("  ", "--help, -h", "Show help"));
    help.push_str(&line("  ", "--version, -v", "Show version"));
    help.push_str(&line("  ", "--timeout <seconds>", "Set request timeout"));

    help.push('\n');

    help.push_str(&format!("{}\n", "Requests:".yellow().bold()));
    help.push_str(&line("  ", "METHOD <path>", "Start a request"));
    help.push_str(&line("  ", "<name>=<value>", "Query parameter"));
    help.push_str(&line("  ", "<name>: <value>", "Request header"));
    help.push_str(&line("  ", "<blank line>", "Body starts after an empty line"));
    help.push_str(&line("  ", "###", "End of request"));

    help.push('\n');

    help.push_str(&format!("{}\n", "Methods:".yellow().bold()));
    help.push_str(&format!(
        "  {}\n",
        ["GET", "POST", "PUT", "PATCH", "DELETE", "HEAD", "OPTIONS"]
            .join("  ")
            .green()
            .bold()
    ));

    help.push('\n');

    help.push_str(&format!("{}\n", "Example:".yellow().bold()));
    help.push_str("  GET /users?page=1\n");
    help.push_str("  Accept: application/json\n");
    help.push_str("  Authorization: Bearer <token>\n");
    help.push_str("  ###\n");

    help.push('\n');

    help.push_str(&format!("{}\n", "Commands:".yellow().bold()));

    help.push_str(&group("Session"));
    help.push_str(&line("    ", "base <url>", "Set the base URL for relative requests"));
    help.push_str(&line("    ", "timeout <seconds>", "Set the request timeout"));

    help.push_str(&group("Headers"));
    help.push_str(&line("    ", "header set <key> <value>", "Add a global header"));
    help.push_str(&line("    ", "header list", "List global headers"));
    help.push_str(&line("    ", "header remove <key>", "Remove a global header"));
    help.push_str(&line("    ", "header clear", "Remove all global headers"));

    help.push_str(&group("Variables"));
    help.push_str(&line("    ", "var set <name> <value>", "Store a variable"));
    help.push_str(&line("    ", "var list", "List variables"));
    help.push_str(&line("    ", "var remove <name>", "Remove a variable"));
    help.push_str(&line("    ", "var clear", "Remove all variables"));

    help.push_str(&group("Saved Requests (req, alias: request)"));
    help.push_str(&line("    ", "req save <name>", "Save the last executed request"));
    help.push_str(&line("    ", "req run <name>", "Run a saved request"));
    help.push_str(&line("    ", "req list", "List saved requests"));
    help.push_str(&line("    ", "req show <name>", "Show a saved request"));
    help.push_str(&line("    ", "req rename <old> <new>", "Rename a saved request"));
    help.push_str(&line("    ", "req remove <name>", "Remove a saved request"));
    help.push_str(&line("    ", "req clear", "Remove all saved requests"));
    help.push_str(&line(
        "    ",
        "req save-response <path>",
        "Save the last response body to a file",
    ));

    help.push_str(&group("History"));
    help.push_str(&line("    ", "history [list]", "List command history"));
    help.push_str(&line("    ", "history show <id>", "Show a history entry"));
    help.push_str(&line("    ", "history rerun <id>", "Rerun a history entry"));
    help.push_str(&line("    ", "history clear", "Clear command history"));

    help.push_str(&group("Shell"));
    help.push_str(&line("    ", "clear", "Clear the terminal screen"));
    help.push_str(&line("    ", "version", "Show the version"));
    help.push_str(&line("    ", "help", "Show this help"));
    help.push_str(&line("    ", "exit", "Exit the REPL"));

    help.push_str(&rule());

    help
}

fn rule() -> String {
    format!("{}\n", RULE.repeat(50).dimmed())
}

fn group(name: &str) -> String {
    format!("  {}:\n", name.bold().cyan())
}

fn line(indent: &str, name: &str, description: &str) -> String {
    format!(
        "{indent} {} {}\n",
        format!("{name:<28}").green().bold(),
        description
    )
}
