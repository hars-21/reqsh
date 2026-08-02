use std::time::Duration;

#[derive(Debug)]
pub enum Command {
    Http(RequestSpec),
    Session(SessionCommand),
    Variable(VariableCommand),
    Request(RequestCommand),
    History(HistoryCommand),
    Shell(ShellCommand),
}

#[derive(Debug)]
pub enum SessionCommand {
    Base(String),
    Header(HeaderCommand),
    Timeout(Duration),
}

#[derive(Debug)]
pub enum HeaderCommand {
    Set { name: String, value: String },
    List,
    Remove { name: String },
    Clear,
}

#[derive(Debug)]
pub enum VariableCommand {
    Set { name: String, value: String },
    List,
    Remove { name: String },
    Clear,
}

#[derive(Debug)]
pub enum HistoryCommand {
    Show { index: usize },
    List,
    Remove { index: usize },
    Clear,
    Rerun { index: usize },
}

#[derive(Debug)]
pub enum ShellCommand {
    Help,
    Version,
    Clear,
    Exit,
}

#[derive(Debug)]
pub enum RequestCommand {
    Save { name: String },
    Run { name: String },
    List,
    Show { name: String },
    Rename { old_name: String, new_name: String },
    Remove { name: String },
    Clear,
}

#[derive(Debug)]
pub struct RequestSpec {
    pub method: Method,
    pub path: String,
    pub headers: Vec<Header>,
    pub queries: Vec<Query>,
    pub body: String,
}

#[derive(Debug)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
}

#[derive(Debug)]
pub struct Header {
    pub name: String,
    pub value: String,
}

#[derive(Debug)]
pub struct Query {
    pub name: String,
    pub value: String,
}
