# Development

This document explains how to work on Copy for AI locally.

## Current status

The project is in foundation setup.

App code will be added in future pull requests.

## Requirements

Requirements will be added as the stack is introduced.

Planned stack:

- Tauri v2
- Rust
- React
- TypeScript
- Python converter sidecar

## Branch workflow

Create a feature branch for each change:

```bash
git checkout main
git pull origin main
git checkout -b feature/my-change
```

Commit with a clear message:

``git commit -m "feat: add clipboard copy command"``

Push the branch:

``git push -u origin feature/my-change``

Open a pull request into ``main``.

### Pull request checks

Every pull request should pass:

- Formatting
- Linting
- Tests
- Build

These checks will be expanded as the project grows.


## Running the CLI

```bash
cargo run -p copy-for-ai -- convert ./sample.txt
```
Running tests

``cargo test``

Formatting

``cargo fmt``

Linting
```
cargo clippy --all-targets --all-features
```
