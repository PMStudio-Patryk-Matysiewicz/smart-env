# smart-env

[![Crates.io](https://img.shields.io/crates/v/smart-env.svg)](https://crates.io/crates/smart-env)
[![Documentation](https://docs.rs/smart-env/badge.svg)](https://docs.rs/smart-env)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

A modern, type-safe, and zero-boilerplate environment configuration loader for Rust, powered by procedural derive macros.

`smart-env` parses environment variables and `.env` files into strongly-typed Rust structures with rich compile-time validation, fallback defaults, and thread-safe testability.

---

## Features

- **Zero Boilerplate**: Annotate your struct with `#[derive(Config)]` and call `AppConfig::load()` or `AppConfig::try_load()`.
- **Full Primitive Type Support**: Built-in support for all Rust primitive types (`u8`–`u128`, `i8`–`i128`, `usize`, `isize`, `f32`, `f64`, `bool`, `char`, `String`).
- **Optional Fields (`Option<T>`)**: Missing optional variables automatically evaluate to `None` without throwing errors.
- **Field Attributes**:
  - `#[config(default = "...")]` – Fallback values parsed and validated directly into target types.
  - `#[config(env = "...")]` – Explicitly map fields to custom environment keys.
- **Fail-Safe Loading**: Choose between panicking on launch (`load()`) or safe error propagation (`try_load() -> Result<Self, EnvError>`).
- **Deterministic Testability**: Built with `ConfigBuilder::with_vars()` for isolated, thread-safe testing without `unsafe std::env::set_var`.

---

## Installation
*Currently unavailable*
- [x] Add `smart-env` to your `Cargo.toml`:

```toml
[dependencies]
smart-env = "0.1"
```

---

## Quickstart

```rust
use smart_env::{Config, EnvError};

#[derive(Config, Debug, PartialEq)]
struct AppConfig {
    // Automatically mapped to HOST
    host: String,

    // Automatically mapped to PORT, falls back to 8080 if not set
    #[config(default = "8080")]
    port: u16,

    // Custom environment variable mapping
    #[config(env = "DATABASE_URL")]
    db_url: String,

    // Optional field: returns None if DEBUG_MODE is missing
    debug_mode: Option<bool>,
}

fn main() -> Result<(), EnvError> {
    // Safe loading with Result propagation
    let config = AppConfig::try_load()?;

    println!("Server running on {}:{}", config.host, config.port);
    println!("Database endpoint: {}", config.db_url);

    if config.debug_mode.unwrap_or(false) {
        println!("Debug mode is ENABLED");
    }

    Ok(())
}
```

---

## Usage Guide

### 1. Automatic Key Convention

By default, struct field names are converted to uppercase environment variable keys:

| Struct Field | Environment Variable Key |
| :--- | :--- |
| `server_host: String` | `SERVER_HOST` |
| `db_port: u16` | `DB_PORT` |
| `enable_metrics: bool` | `ENABLE_METRICS` |

---

### 2. Custom Key Override (`#[config(env = "...")]`)

Override the default naming convention when matching existing container or legacy environment variables:

```rust
#[derive(Config)]
struct Config {
    #[config(env = "REDIS_PRIMARY_ENDPOINT")]
    redis_url: String,
}
```

---

### 3. Fallback Values (`#[config(default = "...")]`)

Provide default values directly over fields. Default strings are automatically parsed using the field's target type `FromEnv` implementation:

```rust
#[derive(Config)]
struct ServerConfig {
    #[config(default = "127.0.0.1")]
    host: String,

    #[config(default = "3000")]
    port: u16,

    #[config(default = "false")]
    maintenance_mode: bool,
}
```

---

### 4. Optional Environment Variables (`Option<T>`)

Fields wrapped in `Option<T>` will evaluate to `Some(parsed_value)` if present, or `None` if absent:

```rust
#[derive(Config)]
struct TelemetryConfig {
    // Required
    app_id: String,

    // Optional: Returns None if SENTRY_DSN is not set
    sentry_dsn: Option<String>,
}
```

---

## Architecture

`smart-env` is structured as a modular Cargo Workspace:

```
smart-env/
├── core/     (smart-env-core)   -> Engine logic, ConfigBuilder, EnvStore & type parsers
├── macros/   (smart-env-macros) -> Procedural derive macro implementation (syn/quote)
├── cli/      (smart-env-cli)    -> Developer tooling for .env checking and generation
└── src/      (smart-env)        -> Unified public facade crate
```

---

## Roadmap

- [x] Core `ConfigBuilder` & `EnvStore` engine
- [x] Basic `#[derive(Config)]` macro
- [x] Primitive types & `Option<T>` support
- [x] Field attributes (`#[config(default = "...")]`, `#[config(env = "...")]`)
- [x] Safe `try_load()` error handling
- [x] Deterministic `with_vars()` builder testing
- [x] Unified facade crate (`smart-env`)
- [ ] Nested configuration structs (`#[config(nested)]`)
- [ ] Array / Vector parsing (`Vec<T>` with custom separators)
- [ ] CLI `.env.example` generator (`smart-env-cli`)

---

## License

Licensed under either of:

* Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
* MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.