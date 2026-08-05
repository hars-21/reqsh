---
title: Usage
description: Learn how to use reqsh for API testing workflows in the interactive REPL.
order: 3
---

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

Type the HTTP method followed by the path, then terminate the request with a `###` line to execute it.

```sh
reqsh> GET /users
.....> ###
```

### Request Body

Leave a blank line after headers to start writing the body. End the request with `###`.

```sh
reqsh> POST /users
.....> Content-Type: application/json
.....>
.....> {"name": "Alice", "email": "alice@example.com"}
.....> ###
```

### Query Parameters

Add query parameters with `name=value` lines.

```sh
reqsh> GET /users
.....> page=1
.....> limit=20
.....> ###
```

### Absolute URLs

You can use absolute URLs directly without setting a base URL.

```sh
reqsh> GET https://api.github.com/users/hars-21
.....> ###
```

## Headers

### Global Headers

Add persistent headers that apply to all requests in the session.

```sh
reqsh> header set Authorization Bearer sk_test_123
reqsh> header set Content-Type application/json
```

### Per-Request Headers

Add headers to individual requests. They override global headers with the same name.

```sh
reqsh> GET /users
.....> X-Custom-Header: value
.....> ###
```

### View and Remove Headers

```sh
reqsh> header list
reqsh> header remove Authorization
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
