---
title: Commands
description: Full reference for all REPL commands available in reqsh.
order: 4
---

# Commands

Beyond standard HTTP methods (`GET`, `POST`, `PUT`, `PATCH`, `DELETE`, `HEAD`, `OPTIONS`), reqsh provides specific REPL commands to manage your session.

## Request Syntax

Requests are built over multiple lines and ended with a `###` line, which also executes the request.

```
METHOD <path>
<name>=<value>          query parameter
<name>: <value>         request header
<blank line>            body starts after an empty line
<body>
###                     end of request
```

```sh
reqsh> GET /users?page=1
.....> Accept: application/json
.....> Authorization: Bearer <token>
.....> ###
```

HTTP methods are case-insensitive. You can also use absolute URLs without setting a base URL.

## Command Reference

### Session

| Command   | Usage               | Description                               |
| --------- | ------------------- | ----------------------------------------- |
| `base`    | `base <url>`        | Set the global base URL for the session.  |
| `timeout` | `timeout <seconds>` | Set the request timeout for the session.  |
| `clear`   | `clear`             | Clear the terminal screen.                |
| `version` | `version`           | Print the reqsh version.                  |
| `help`    | `help`              | Display syntax and command documentation. |
| `exit`    | `exit`              | Terminate the shell session.              |

### Headers

| Command         | Usage                      | Description                    |
| --------------- | -------------------------- | ------------------------------ |
| `header set`    | `header set <key> <value>` | Add a persistent header.       |
| `header list`   | `header list`              | List all persistent headers.   |
| `header remove` | `header remove <key>`      | Remove a persistent header.    |
| `header clear`  | `header clear`             | Remove all persistent headers. |

### Variables

| Command      | Usage                    | Description           |
| ------------ | ------------------------ | --------------------- |
| `var set`    | `var set <name> <value>` | Store a variable.     |
| `var list`   | `var list`               | List all variables.   |
| `var remove` | `var remove <name>`      | Remove a variable.    |
| `var clear`  | `var clear`              | Remove all variables. |

### Saved Requests

`request` is an alias for `req`.

| Command      | Usage                    | Description                     |
| ------------ | ------------------------ | ------------------------------- |
| `req save`   | `req save <name>`        | Save the last executed request. |
| `req run`    | `req run <name>`         | Execute a saved request.        |
| `req list`   | `req list`               | List all saved requests.        |
| `req show`   | `req show <name>`        | Show a saved request.           |
| `req rename` | `req rename <old> <new>` | Rename a saved request.         |
| `req remove` | `req remove <name>`      | Delete a saved request by name. |
| `req clear`  | `req clear`              | Delete all saved requests.      |

### History

| Command         | Usage                | Description                                 |
| --------------- | -------------------- | ------------------------------------------- |
| `history`       | `history` / `list`   | View the numbered history of past commands. |
| `history show`  | `history show <id>`  | View a single command from history.         |
| `history rerun` | `history rerun <id>` | Re-execute a command from history.          |
| `history clear` | `history clear`      | Clear the command history.                  |

## Save and Run

Save any request after executing it, then replay it instantly.

```sh
reqsh> GET /users/{{id}}
.....> ###
reqsh> req save get-user
saved request: get-user
reqsh> req list
get-user (GET) /users/{{id}}
reqsh> req run get-user
```

## History

View all commands executed in the session.

```sh
reqsh> history
   1: base https://api.example.com
   2: header set Authorization Bearer sk_test
   3: GET /users
```

Rerun a command from history by its ID.

```sh
reqsh> history rerun 3
```

## Variables

Store values and reference them with `{{name}}` syntax.

```sh
reqsh> var set token eyJhbGciOiJIUzI1NiJ9
reqsh> GET /users/{{token}}
.....> ###
```

List and remove variables.

```sh
reqsh> var list
reqsh> var remove token
```

## Timeout

Set a request timeout for all requests in the session.

```sh
reqsh> timeout 10
```

## Help

Display the built-in help with all available commands and syntax.

```sh
reqsh> help
```
