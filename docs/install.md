# Installation

Get reqsh running on your system in seconds. Choose the method that best fits your workflow.

## Install Script

The fastest and recommended way to install. This script detects your OS and architecture, downloads the latest binary and places it in your PATH.

```sh
curl -fsSL https://reqsh.dev/install.sh | sh
```

## Prebuilt Binary

Download the latest binary for your platform from the [GitHub Releases](https://github.com/hars-21/reqsh/releases) page. Available for macOS (Intel & Silicon), Linux (x86_64) and Windows (x86_64).

1. Download the binary for your platform.
2. Move it to a directory included in your system PATH.
3. Grant execution permissions.

```sh
# macOS / Linux
mv reqsh ~/.local/bin/
chmod +x ~/.local/bin/reqsh
reqsh
```

Windows users: place `reqsh.exe` in a directory listed in your `%PATH%`.

## Build from Source

If you want to compile reqsh yourself, you'll need the [Rust toolchain](https://rustup.rs) installed.

```sh
git clone https://github.com/hars-21/reqsh.git
cd reqsh
cargo build --release
```

The binary will be at `target/release/reqsh`.

## Verify Installation

Confirm reqsh is installed and working correctly.

```sh
reqsh --help
```
