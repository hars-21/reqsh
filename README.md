# reqsh

![Last Update](https://img.shields.io/github/last-commit/hars-21/reqsh?label=Last%20update&style=classic)
![Rust](https://img.shields.io/badge/Made_in-Rust-orange)
![License](https://img.shields.io/badge/license-MIT-brightgreen.svg)

> Interactive HTTP shell for API workflows

reqsh is a lightweight, terminal-first tool to send HTTP requests, save them, and reuse them — all in an interactive REPL.

## Features

- Interactive REPL (`reqsh>`)
- Run HTTP requests (`GET`, `POST`)
- Save and reuse requests
- Command history & autocomplete
- Simple environment support

## Usage

Start the shell:

```bash
reqsh
```

Example:

```bash
reqsh> GET https://api.example.com/users
reqsh> save getUsers
reqsh> run getUsers
```

## Commands

- `GET <url>` — send GET request
- `POST <url>` — send POST request
- `save <name>` — save last request
- `run <name>` — run saved request
- `list` — list saved requests
- `delete <name>` — delete request
- `set base_url <url>` — set base URL
- `headers set <name> <key> <value>` - set headers for saved requests
- `headers clear` - clear all headers
- `exit` — quit
- `help` — help

## License

MIT
