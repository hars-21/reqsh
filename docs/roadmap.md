# Roadmap

## v0.3.0 - Request Management & Lifecycle

Major focus on making requests work properly with full lifecycle support.

### Features

- [ ] Full CRUD for requests and responses (save, load, list, delete)
- [ ] Improve request execution performance
- [ ] Better UX for request-response flow
- [ ] Store request history during session
- [ ] Save responses to files
- [ ] Basic request replay from history

### Fixes

- [ ] Single client per session (right now it builds separate client for each request, need to fix this)
- [ ] Handle connection errors gracefully
- [ ] Better error messages when request fails

## v0.4.0 - Configuration & Personalization

Add config support so users can customize how reqsh works.

### Features

- [ ] Config file support
- [ ] Default config values (timeouts, default headers, etc.)
- [ ] User-defined config options
- [ ] Customization (custom prompt, colors, default method, etc.)
- [ ] `config set` and `config get` commands in CLI
- [ ] Improved storage backend (use config for path, format, etc.)

### Fixes

- [ ] Storage bugs from v0.3.0
- [ ] Config validation

## v0.5.0 — CLI Polish & Environments

Polish the CLI experience and add environment support.

### Features

- [ ] Better CLI output formatting (tables, colors, etc.)
- [ ] Tab completion improvements
- [ ] Environment support (dev, prod, test, stage, etc.)
- [ ] Variable management - set, get, unset, list
- [ ] Secret management - store tokens, passwords securely
- [ ] `-env` flag to switch environments
- [ ] Environment specific config overrides
- [ ] .env file support
