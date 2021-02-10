# mediatap

A command line tool to automate downloading TV-shows/movies from online media plattforms of German public broadcasting (e.g. ARD, ZDF, WDR, etc...).

## Installation

Right now, there sadly is no user-friendly way of installing mediatap. You'll need to install Rust (https://www.rust-lang.org/learn/get-started) and install `mediatap` via `cargo`, Rust's package manager and build tool.

```shell
cargo install mediatap
```

## Development

### Prerequisites

The following tools need to be installed:

- Rust (using `rustup` if possible)
- The `diesel` CLI => `cargo install diesel_cli`

### Database migrations

Since `mediatap` is using SQLite as a database backend, you have to supply the path to the database file each time you make a migration with the `diesel` CLI. The following command simplifies this task massively:

```shell
diesel migration run --database-url "$(cargo run -- emit-database-path)"
```

Notice that the `emit-database-path` subcommand is not available in release mode.
