# Rust Template [![Github Actions][gha-badge]][gha] [![License: MIT][license-badge]][license]

[gha]: https://github.com/paulrberg/rust-template/actions
[gha-badge]: https://github.com/paulrberg/rust-template/actions/workflows/ci.yml/badge.svg
[license]: https://opensource.org/licenses/MIT
[license-badge]: https://img.shields.io/badge/License-MIT-blue.svg

A template for developing Rust projects, with sensible defaults.

## Getting Started

Click the [`Use this template`](https://github.com/paulrberg/rust-template/generate) button at the top of the page to create a new repository with this repo as the initial state.

## Features

### Sensible Defaults

This template comes with sensible default configurations in the following files:

```text
├── .editorconfig
├── .gitignore
├── .prettierrc.yml
├── Cargo.toml
└── rustfmt.toml
```

### GitHub Actions

This template comes with GitHub Actions pre-configured. Your code will be linted and tested on every push and pull
request made to the `main` branch.

You can edit the CI script in [.github/workflows/ci.yml](./.github/workflows/ci.yml).

## Usage

See [The Rust Book](https://doc.rust-lang.org/book/) and [The Cargo Book](https://doc.rust-lang.org/cargo/index.html).

## License

[MIT](./LICENSE.md) © Paul Razvan Berg
