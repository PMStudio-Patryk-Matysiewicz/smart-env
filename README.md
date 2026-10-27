# smart-env

Ergonomic, type-safe configuration for Rust applications. smart-env reads variables from the
system environment and `.env` files, decides which source wins, parses values into Rust types
and tells you exactly what is missing or wrong.

> **Status: early development (0.1.0).** The core library works and is tested. The
> `#[derive(Config)]` macro and the CLI are planned and not usable yet. Not published on
> crates.io.

## Features

- **Simple API:** `get`, `get_or` and `init` for the common case.
- **Builder:** `ConfigBuilder` combines `.env` files and the system environment; sources loaded
  later override earlier ones.
- **Typed values:** `String`, `bool`, `u16` and `u32` out of the box (`FromEnv` trait).
- **`explain`:** shows where a value came from (`System`, `EnvFile` or `Missing`).
- **Readable errors:** `EnvError` implements `miette::Diagnostic`, with a code and a hint.

## Installation

```toml
[dependencies]
smart-env-core = { git = "https://github.com/PMStudio-Patryk-Matysiewicz/smart-env" }
```

## Usage

### Simple API

```rust
use smart_env_core::{explain, get, get_or, init};

fn main() {
    // Load .env once, at startup
    init();

    // System environment wins over .env
    let port: u16 = get_or("PORT", 8080);
    let debug: bool = get("DEBUG").unwrap_or(false);

    // Where did the value come from?
    let info = explain("PORT");
    println!("PORT={port} DEBUG={debug} (source: {:?})", info.source);
}
```

### Builder

```rust
use smart_env_core::ConfigBuilder;

fn main() -> miette::Result<()> {
    // Later sources override earlier ones
    let store = ConfigBuilder::new()
        .load_env_file(".env")
        .load_system_env()
        .build();

    let port: u16 = store.get_parsed("PORT")?;
    let url: String = store.get_parsed("DATABASE_URL")?;

    println!("port {port}, database {url}");
    Ok(())
}
```

With `miette` (`features = ["fancy"]`) a missing variable is reported like this:

```text
Error: smart_env::missing_var

  × Missing required environment variable: DATABASE_URL
  help: Make sure this variable is exported in your system or defined in
        the .env file.
```

## `.env` format

```env
# comments and empty lines are skipped
PORT=3000
DEBUG=true
```

One `KEY=VALUE` per line, whitespace around keys and values is trimmed. Quotes and inline
comments are not handled yet. A missing file is treated as empty.

## Good to know

- `get` returns `None` both when a variable is missing and when it cannot be parsed. Use
  `ConfigBuilder` and `get_parsed` when you need the reason.
- `init` loads `.env` from the current directory once per process; later calls do nothing.

## Project layout

| Crate | Purpose |
| --- | --- |
| `core` (`smart-env-core`) | the library |
| `macros` | `#[derive(Config)]`, work in progress |
| `cli` | command line tool, work in progress |

## Roadmap

- `#[derive(Config)]`: load and parse values straight into your structs, no boilerplate
- CLI commands
- more types and a fuller `.env` syntax

## Running tests

```bash
cd smart-env
cargo test -p smart-env-core
```

Unit tests live next to the code in `src/`; tests of the public API live in `core/tests/`.
