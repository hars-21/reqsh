<p align="center">
  <img src="assets/banner.svg" alt="reqsh">
</p>

[![Last Update](https://img.shields.io/github/last-commit/hars-21/reqsh?label=last%20update)](https://github.com/hars-21/reqsh)
[![Rust](https://img.shields.io/badge/rust-v1.93.0-orange)]()
[![License](https://img.shields.io/badge/license-MIT-brightgreen.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-under%20development-yellow)]()

Interactive HTTP shell for API workflows. Send HTTP requests, manage headers and base URLs, and rerun past commands from a terminal REPL.

## Features

- Interactive REPL with tab completion
- Send GET, POST, PUT, DELETE requests
- Multi-line request input for custom headers and body
- Persistent session state (base URL, global headers, variables)
- Variable interpolation with `{{name}}` syntax in paths, headers, and body
- Query parameter support with `param: key=value` lines
- Save and run requests in-session
- JSON response pretty-printing
- Command history and rerun by index
- Colored terminal output

## Installation

### Install script

```bash
curl -fsSL https://raw.githubusercontent.com/hars-21/reqsh/main/install.sh | sh
```

### Pre-built binary

Download the latest binary from the [releases page](https://github.com/hars-21/reqsh/releases/latest).

### Build from source

```bash
git clone https://github.com/hars-21/reqsh.git
cd reqsh
cargo build --release
```

The binary will be at `target/release/reqsh`.

## Usage

CLI Options:

```bash
reqsh --help
reqsh --version
```

Start the REPL:

```bash
reqsh
```

Set a base URL:

```bash
reqsh> base https://api.example.com
```

Send a GET request:

```bash
reqsh> GET /users
```

Send a POST request with headers and body:

```bash
reqsh> POST /users
.....> Content-Type: application/json
.....> Authorization: Bearer token123
.....>
.....> {"name": "john"}
.....> ::send
```

## Commands

| Command                | Description                          |
| ---------------------- | ------------------------------------ |
| `GET <path>`           | Send GET request                     |
| `POST <path>`          | Send POST request                    |
| `PUT <path>`           | Send PUT request                     |
| `DELETE <path>`        | Send DELETE request                  |
| `base <url>`           | Set base URL for all requests        |
| `header <key> <value>` | Set a global header for all requests |
| `set <name> <value>`   | Set a session variable               |
| `unset <name>`         | Remove a session variable            |
| `unset header <key>`   | Remove a global header               |
| `save <name>`          | Save the last request to memory      |
| `run <name>`           | Execute a saved request              |
| `requests`             | List saved requests                  |
| `vars`                 | List session variables               |
| `headers`              | List global headers                  |
| `history`              | Show command history                 |
| `rerun <index>`        | Re-run a command from history        |
| `help`                 | Show built-in help                   |
| `exit`                 | Exit the REPL                        |

### Multi-line requests

When you type a method and path, the REPL enters multi-line mode (`.....>`). You can add headers as `key: value` pairs, followed by a blank line and the request body. End with `::send` on its own line.

Query parameters can be added with `param:` lines:

```bash
reqsh> GET /users
.....> param: page=1
.....> param: limit=20
.....> ::send
```

Variables set with `set` are interpolated at request time using `{{name}}`:

```bash
reqsh> set token eyJhbGciOiJIUzI1NiJ9
reqsh> GET /users/{{token}}
.....> Authorization: Bearer {{token}}
.....> ::send
```

## Contributing

Contributions are welcome. Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines on how to set up the project, run tests, and submit changes.

## License

MIT. See [LICENSE](LICENSE) for details.
