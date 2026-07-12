# Commands

Beyond standard HTTP methods (`GET`, `POST`, `PUT`, `PATCH`, `DELETE`, `HEAD`, `OPTIONS`), reqsh provides specific REPL commands to manage your session.

## Command Reference

| Command        | Usage                  | Description                                                                    |
| -------------- | ---------------------- | ------------------------------------------------------------------------------ |
| `base`         | `base <url>`           | Set the global base URL for the session.                                       |
| `header`       | `header <key> <value>` | Add a persistent header applied to all requests.                               |
| `set`          | `set <name> <value>`   | Store a variable for interpolation.                                            |
| `unset`        | `unset <name>`         | Remove a variable.                                                             |
| `unset header` | `unset header <key>`   | Remove a global header.                                                        |
| `save`         | `save <name>`          | Save the last executed request to memory.                                      |
| `remove`       | `remove <name>`        | Delete a saved request by name.                                                |
| `run`          | `run <name>`           | Execute a saved request.                                                       |
| `vars`         | `vars`                 | List all session variables.                                                    |
| `headers`      | `headers`              | List all global headers.                                                       |
| `requests`     | `requests`             | List all saved requests.                                                       |
| `history`      | `history`              | View the numbered history of past commands.                                    |
| `rerun`        | `rerun <id>`           | Instantly re-execute a request from history.                                   |
| `timeout`      | `timeout <seconds>`    | Set the request timeout for the session.                                       |
| `clear`        | `clear`                | Reset the entire session state (base URL, headers, variables, saved requests). |
| `help`         | `help`                 | Display syntax and command documentation.                                      |
| `exit`         | `exit`                 | Terminate the shell session.                                                   |

## Session Management

### Save and Run

Save any request after executing it, then replay it instantly.

```sh
reqsh> GET /users/{{id}}
.....> ::send
reqsh> save get-user
saved
reqsh> run get-user
```

### History

View all commands executed in the current session.

```sh
reqsh> history
1: base https://api.example.com
2: header Authorization Bearer sk_test
3: GET /users
```

### Rerun

Re-execute a command from history by its ID.

```sh
reqsh> rerun 3
```

## Variables

### Set and Use

Store values and reference them with `{{name}}` syntax.

```sh
reqsh> set token eyJhbGciOiJIUzI1NiJ9
reqsh> GET /users/{{token}}
.....> ::send
```

### List and Remove

```sh
reqsh> vars
reqsh> unset token
```

## Timeout

Set a request timeout for all requests in the session.

```sh
reqsh> timeout 10
Request timeout set to 10 seconds
```

## Clear

Reset the entire session, base URL, headers, variables and saved requests.

```sh
reqsh> clear
Session cleared
```

## Help

Display the built-in help with all available commands and syntax.

```sh
reqsh> help
```
