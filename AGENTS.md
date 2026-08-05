# Development Instructions

AI agents working on this Rust project must follow these guidelines.

References:

- **Project overview**: @README.md
- **Dependencies**: @Cargo.toml

## Validation

After changing Rust code, run these commands in order:

1. `just full-check` — verify formatting and run Clippy with warnings denied.
2. `just test` — run the test suite.

If the formatting check fails, run `just full-write`, then rerun the validation commands. Run `just --list` to see the
available recipes.
