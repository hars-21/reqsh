<p align="center">
  <img src="assets/banner.svg" alt="reqsh">
</p>

[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey)](#)
[![Rust](https://img.shields.io/badge/rust-v1.93.0-orange)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-under%20development-yellow)]()
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](http://makeapullrequest.com)

Interactive HTTP shell for API workflows. Send HTTP requests, manage headers and base URLs, and rerun past commands from a terminal REPL.

## Features

- Interactive REPL with tab completion
- Send GET, POST, PUT, PATCH, DELETE, HEAD, OPTIONS requests (case-insensitive)
- Multi-line request input for custom headers and body
- Persistent session state across restarts (base URL, headers, variables, saved requests)
- Variable interpolation with `{{name}}` in paths, headers, body, and query params
- Query parameter support with `param: key=value` lines
- Save and run requests in-session
- HTTP version, status code, headers and body in responses
- JSON response pretty-printing with colored output
- Command history and rerun by index
- Tab completion for commands, saved requests, variables and headers
- Configurable request timeout
- Colored terminal output

## Installation

### Install script

```bash
curl -fsSL https://raw.githubusercontent.com/hars-21/reqsh/main/install.sh | sh
```

### Pre-built binary

Download the latest binary for your platform from the [releases page](https://github.com/hars-21/reqsh/releases/latest). macOS (Intel & Silicon), Linux (x86_64), and Windows (x86_64) are available.

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
reqsh               # Start the REPL
reqsh --help -h     # Show help
reqsh --version -v  # Show version
reqsh --timeout 30  # Start REPL with a default request timeout
```

Start the REPL:

```bash
reqsh
```

Set a base URL (optional — you can use absolute URLs directly):

```bash
reqsh> base https://api.example.com
```

Send a GET request (relative path requires a base URL):

```bash
reqsh> GET /users
```

Or use an absolute URL directly:

```bash
reqsh> GET https://jsonplaceholder.typicode.com/posts
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
| `GET <url>`            | Send GET request                     |
| `POST <url>`           | Send POST request                    |
| `PUT <url>`            | Send PUT request                     |
| `PATCH <url>`          | Send PATCH request                   |
| `DELETE <url>`         | Send DELETE request                  |
| `HEAD <url>`           | Send HEAD request                    |
| `OPTIONS <url>`        | Send OPTIONS request                 |
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
| `timeout <seconds>`    | Set request timeout for the session  |
| `clear`                | Reset session state entirely         |
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
