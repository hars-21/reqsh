![reqsh | Interactive HTTP shell](assets/banner.png)

[![Platform](https://img.shields.io/badge/platform-macOS%20%7C%20Linux%20%7C%20Windows-lightgrey)](#)
[![Rust](https://img.shields.io/badge/rust-v1.93.0-orange)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)
[![Status](https://img.shields.io/badge/status-under%20development-yellow)]()
[![PRs Welcome](https://img.shields.io/badge/PRs-welcome-brightgreen.svg)](http://makeapullrequest.com)

Interactive HTTP shell for API workflows. Send requests, manage headers & variables and rerun past commands from a terminal REPL.

## Features

- Interactive REPL with multi-line request editing
- Send GET, POST, PUT, PATCH, DELETE, HEAD, OPTIONS requests (case-insensitive)
- Multi-line request input with query params, headers, body and `###` terminator
- Persistent session state across restarts (`~/.reqsh_state.json`)
- Variable interpolation with `{{name}}` syntax in paths, headers, query params and bodies
- Global headers and variables (`header`, `var`)
- Save, manage and run requests in-session (`req`)
- Re-run the last executed request (`req rerun`)
- Save the last response body to a file (`req save-response <path>`)
- Command history with show, rerun and clear
- Configurable request timeout
- JSON response pretty-printing with colored output

## Quick Start

### Install

```bash
curl -fsSL https://reqsh.dev/install.sh | sh
```

Or download a binary from the [releases page](https://github.com/hars-21/reqsh/releases/latest) or [build from source](docs/install.md).

### Usage

```bash
reqsh> base https://api.example.com
reqsh> GET /users
.....> ###

reqsh> POST /users
.....> Content-Type: application/json
.....>
.....> {"name": "john"}
.....> ###
```

For full documentation on commands, variables and usage see the [docs](docs/introduction.md).

## Contributing

Contributions are welcome. Please read [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

MIT. See [LICENSE](LICENSE) for details.
