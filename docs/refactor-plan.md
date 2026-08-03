# Refactor Plan

## Goals

- Improve architecture without changing behavior.
- Learn Rust by understanding ownership and APIs.
- Introduce tests before large refactors.
- Keep the project working after every commit.

## Rules

- One responsibility per commit.
- No unnecessary clones.
- No premature traits.
- Design before implementation.
- Refactor, don't rewrite.

## Milestones

- [x] Introduce Repl
- [ ] Move event loop
- [x] Introduce Reader
- [x] Introduce Session
- [x] Introduce Command AST
- [x] Introduce Parser
- [x] Introduce Executor
- [x] Introduce Client
