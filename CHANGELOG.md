# Changelog

## [v0.2.0](https://github.com/hars-21/reqsh/compare/v0.1.5..v0.2.0) - 2026-07-09



### 🚀 Features
- Persist session state to `~/.reqsh_state.json` in [#20](https://github.com/hars-21/reqsh/pull/20)

### ❤️ New Contributors

* @Bircoder432 made their first contribution in [#20](https://github.com/hars-21/reqsh/pull/20)
## [v0.1.5](https://github.com/hars-21/reqsh/compare/v0.1.4..v0.1.5) - 2026-06-23



### 🚀 Features
- Smart tab completion in [#19](https://github.com/hars-21/reqsh/pull/19)
- Added `--timeout` to stop indefinite requests in [#18](https://github.com/hars-21/reqsh/pull/18)
- New builtin `clear` to clean the session state in [#16](https://github.com/hars-21/reqsh/pull/16)
- Added `HEAD` and `OPTIONS` request methods in [#14](https://github.com/hars-21/reqsh/pull/14)

### 🐛 Bug Fixes
- Auto set `Content-Type` header in [#17](https://github.com/hars-21/reqsh/pull/17)
- `CTRL-C` / `EOF` loses command history in [#15](https://github.com/hars-21/reqsh/pull/15)
## [v0.1.4](https://github.com/hars-21/reqsh/compare/v0.1.3..v0.1.4) - 2026-06-13



### 🚀 Features
- Request methods are now case insensitive in [#3](https://github.com/hars-21/reqsh/pull/3)

### 🐛 Bug Fixes
- Support multi-word header values

### ❤️ New Contributors

* @akglaza made their first contribution in [#3](https://github.com/hars-21/reqsh/pull/3)

* @bakkdoor made their first contribution
## [v0.1.3](https://github.com/hars-21/reqsh/compare/v0.1.2..v0.1.3) - 2026-06-09



### 🚀 Features
- Added response time

### 🐛 Bug Fixes
- Support for absolute URL and remove header override
## [v0.1.2](https://github.com/hars-21/reqsh/compare/v0.1.1..v0.1.2) - 2026-06-06



### 🚀 Features
- Added requests command to view all the saved requests
- New builtin commands - save and run requests
- Added support for query params
- New builtins: unset, headers, vars
- In memory variables and interpolation
- Installation script
## [v0.1.1](https://github.com/hars-21/reqsh/compare/v0.1.0..v0.1.1) - 2026-05-26



### 🚀 Features
- Version and help commands
## [v0.1.0] - 2026-05-24



### 🚀 Features
- Feat: history file is now served from root
- Pretty print json response body
- Pretty print and colorized response
- New builtins history and rerun
- Full response display with headers, status code, version
- Global headers and history file rename
- Feat: initial lexer implementation
- Added support for DELETE requests
- Added support for PUT requests
- Added env variable commands
- Added support for headers
- Added POST method support
- New builtin commands and in memory context

### ❤️ New Contributors

* @hars-21 made their first contribution
