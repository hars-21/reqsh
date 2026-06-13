# Changelog

## 0.1.4 (2026-06-13)

- `PATCH` method support added
- HTTP methods are now case-insensitive (`GET`, `get`, `Get` all work)
- Errors are printed to stderr when output is piped, stdout in interactive mode
- Help text is more compact (fewer blank lines)
- Invalid header names/values now show clear error messages instead of panicking
- Empty input guard added to prevent crash on blank lines in multi-line mode

## 0.1.3 (2026-06-09)

- Response time displayed with each request
- Windows binary support (x86_64-pc-windows-msvc)
- Absolute URLs now work (not just relative paths with `base`)

## 0.1.2 (2026-06-06)

- Variable interpolation with `{{name}}` syntax in paths, headers, and body
- Query parameter support with `param: key=value` in request definitions
- `save <name>` - save a request to session memory
- `run <name>` - execute a saved request
- `requests` - list all saved requests
- `unset <name>` / `unset header <key>` - remove variables and headers
- Added `set`, `unset`, `save`, `run`, `vars`, `headers`, `requests` to tab completion

## 0.1.1 (2026-05-26)

- Added `--version`, `-v` flags to display version
- Added `--help`, `-h` flags to display help text

## 0.1.0 (2026-05-24)

- Initial release
- Interactive REPL with tab completion
- Send GET, POST, PUT, DELETE requests
- Multi-line request input for headers and body
- Persistent session state (base URL, global headers)
- JSON response pretty-printing
- Command history and rerun by index
- Colored terminal output
