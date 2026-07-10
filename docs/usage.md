# Usage

Learn how to use reqsh effectively for your API testing workflow.

## Starting a Session

Launch reqsh by typing `reqsh` in your terminal. You'll be dropped into the interactive REPL with a prompt.

```sh
reqsh
reqsh>
```

## Setting a Base URL

Use the `base` command to set a base URL. All subsequent requests will be relative to this URL.

```sh
reqsh> base https://api.example.com
```

## Making Requests

Type the HTTP method followed by the path. Use `::send` to execute.

```sh
reqsh> GET /users
.....> ::send
```

### Request Body

Leave a blank line after headers to start writing the body.

```sh
reqsh> POST /users
.....> Content-Type: application/json
.....>
.....> {"name": "Alice", "email": "alice@example.com"}
.....> ::send
```

### Query Parameters

Add query parameters with `param:` lines.

```sh
reqsh> GET /users
.....> param: page=1
.....> param: limit=20
.....> ::send
```

### Absolute URLs

You can use absolute URLs directly without setting a base URL.

```sh
reqsh> GET https://api.github.com/users/hars-21
.....> ::send
```

## Headers

### Global Headers

Add persistent headers that apply to all requests in the session.

```sh
reqsh> header Authorization Bearer sk_test_123
reqsh> header Content-Type application/json
```

### Per-Request Headers

Add headers to individual requests.

```sh
reqsh> GET /users
.....> X-Custom-Header: value
.....> ::send
```

### View and Remove Headers

```sh
reqsh> headers
reqsh> unset header Authorization
```

## Response Handling

After each request, reqsh displays:

- HTTP version (e.g., `HTTP/1.1`)
- Status code and status text (color-coded: green for 2xx, yellow for 4xx, red for 5xx)
- Response time in milliseconds
- Full response headers
- Pretty-printed JSON body (auto-detected from `Content-Type`) or raw text

```sh
HTTP/1.1 200 OK 142ms
content-type: application/json
date: Mon, 01 Jan 2024 00:00:00 GMT

{
  "id": 1,
  "name": "Alice"
}
```
