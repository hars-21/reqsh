# reqsh Documentation

Master the interactive HTTP shell. Learn how to install, configure your environment and execute requests efficiently.

## Getting Started

### 1. Install reqsh

Head over to the [installation guide](/docs/install) to download the binary or build from source.

### 2. Start the shell

Simply type `reqsh` in your terminal. This drops you into the interactive REPL.

```sh
reqsh
```

### 3. Set a base URL

Define your target host. All subsequent requests in this session will be appended to this base URL automatically.

```sh
reqsh> base https://api.example.com
```

### 4. Send a request

Type the HTTP method and the relative path. Then execute it using the special `::send` command.

```sh
reqsh> GET /users
.....> ::send
```

## Sending Requests

The shell supports building complex requests step-by-step. Start with the method and path. HTTP methods are case-insensitive - `GET`, `POST`, `PUT`, `PATCH`, `DELETE`, `HEAD`, `OPTIONS` are all supported. You can also use absolute URLs, no base URL required. Add headers on the following lines. Leave a blank line to start writing the body.

```sh
reqsh> PATCH /users/1
.....> Content-Type: application/json
.....>
.....> {"name": "Alice"}
.....> ::send
```

The response includes the HTTP version, status code (color-coded), response time, all response headers and the body. JSON responses are automatically pretty-printed with colored syntax.

```sh
HTTP/1.1 200 OK 142ms
content-type: application/json
date: Mon, 01 Jan 2024 00:00:00 GMT

{
  "id": 1,
  "name": "Alice"
}
```

Session state (base URL, headers, variables, saved requests) is persisted automatically to `~/.reqsh_state.json` and restored when you restart the REPL. Command history is saved to `~/.reqsh_history`.

## Variables

Store values with `set` and reference them anywhere in your request using `{{name}}`. Variables are interpolated at request time.

```sh
reqsh> set token eyJhbGciOiJIUzI1NiJ9
reqsh> set host api.example.com
reqsh> GET /users/{{token}}
.....> Authorization: Bearer {{token}}
.....> ::send
```

## Query Parameters

Add query parameters with `param:` lines. Values are URL-encoded automatically.

```sh
reqsh> GET /users
.....> param: page=1
.....> param: limit=20
.....> ::send
```

## Save & Run

## Timeout

Set a default timeout for all requests in a session.

```sh
reqsh> timeout 10
Request timeout set to 10 seconds
```

You can also set a timeout when starting the REPL:

```sh
reqsh --timeout 30
```

## Clear Session

Reset the entire session state — base URL, headers, variables, and saved requests.

```sh
reqsh> clear
Session cleared
```

## Session Persistence

Your session state is automatically saved to `~/.reqsh_state.json` when you exit and restored when you start the REPL again.

## Save & Run

Save a request to session memory after executing it, then run it again instantly without retyping.

```sh
reqsh> GET /users/{{id}}
.....> ::send
reqsh> save get-user
saved
reqsh> run get-user
```
