# Variables

Variables let you store reusable values and interpolate them into requests using `{{name}}` syntax.

## Setting Variables

Use the `set` command to store a value.

```sh
reqsh> set token eyJhbGciOiJIUzI1NiJ9
reqsh> set base_url https://api.example.com
reqsh> set user_id 42
```

## Using Variables

Reference variables anywhere in your request using double curly braces.

```sh
reqsh> GET /users/{{user_id}}
.....> Authorization: Bearer {{token}}
.....> ::send
```

Variables work in:

- **Paths**: `GET /users/{{user_id}}`
- **Headers**: `Authorization: Bearer {{token}}`
- **Bodies**: `{"id": "{{user_id}}"}`
- **Query params**: `param: page={{page}}`

## Listing Variables

View all stored variables in the current session.

```sh
reqsh> vars
token = eyJhbGciOiJIUzI1NiJ9
base_url = https://api.example.com
user_id = 42
```

## Removing Variables

Remove a variable when it's no longer needed.

```sh
reqsh> unset token
```

## Example Workflow

```sh
reqsh> set token eyJhbGciOiJIUzI1NiJ9
reqsh> set api https://api.example.com
reqsh> GET {{api}}/users/{{token}}
.....> ::send
HTTP/1.1 200 OK 142ms
content-type: application/json

{ ... }
```
