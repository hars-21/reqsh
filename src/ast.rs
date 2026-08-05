use std::fmt;
use std::time::Duration;

use serde::{Deserialize, Serialize};

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
    SaveResponse { path: String },
    Rerun,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestSpec {
    pub method: Method,
    pub path: String,
    pub headers: Vec<Header>,
    pub queries: Vec<Query>,
    pub body: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Method {
    Get,
    Post,
    Put,
    Delete,
    Patch,
    Head,
    Options,
}

impl Method {
    pub fn as_str(&self) -> &'static str {
        match self {
            Method::Get => "GET",
            Method::Post => "POST",
            Method::Put => "PUT",
            Method::Delete => "DELETE",
            Method::Patch => "PATCH",
            Method::Head => "HEAD",
            Method::Options => "OPTIONS",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Header {
    pub name: String,
    pub value: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Query {
    pub name: String,
    pub value: String,
}

impl fmt::Display for RequestSpec {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} {}", self.method.as_str(), self.path)?;

        if !self.queries.is_empty() {
            writeln!(f)?;
            writeln!(f, "queries:")?;
            for query in &self.queries {
                writeln!(f, "  {} = {}", query.name, query.value)?;
            }
        }

        if !self.headers.is_empty() {
            writeln!(f)?;
            writeln!(f, "headers:")?;
            for header in &self.headers {
                writeln!(f, "  {}: {}", header.name, header.value)?;
            }
        }

        if !self.body.is_empty() {
            writeln!(f)?;
            writeln!(f, "body:")?;
            write!(f, "{}", self.body)?;
        }

        Ok(())
    }
}
