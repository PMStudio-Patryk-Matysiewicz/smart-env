# smart-env

[![Crates.io](https://img.shields.io/crates/v/smart-env.svg)](https://crates.io/crates/smart-env)
[![Documentation](https://docs.rs/smart-env/badge.svg)](https://docs.rs/smart-env)
[![License: MIT OR Apache-2.0](https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-blue.svg)](#license)

A type-safe, developer-experience (DX) focused environment configuration loader for Rust with procedural derive macros, automatic prefixing, `Vec<T>` parsing, and built-in CLI diagnostic tools.

---

## ✨ Features

- 🚀 **Zero Boilerplate**: Derive `Config` on your struct to automatically load and parse `.env` files and system environment variables.
- ⚙️ **Type-Safe Parsing**: Built-in support for primitives, `Option<T>`, `Vec<T>`, custom parsed types, and fallbacks.
- 🧱 **Nested Configurations**: Nest config structs with automatic or custom prefixing (e.g. `DB_HOST`, `DB_PORT`).
- 🛠️ **Built-in CLI**: Easily inspect, dump, and validate your `.env` configuration with colored sources (`system` vs `.env`).
- 🛡️ **Flexible Loading**: Panic-safe `try_load()` returning detailed errors alongside standard `load()`.

---

## 📦 Installation

Add `smart-env` to your `Cargo.toml`:

```toml
[dependencies]
smart-env = "0.1.0"
```

---

## 🚀 Quick Start

```rust
use smart_env::Config;

#[derive(Config, Debug, PartialEq)]
struct DatabaseConfig {
    #[config(default = "localhost")]
    host: String,

    #[config(default = "5432")]
    port: u16,
}

#[derive(Config, Debug, PartialEq)]
struct AppConfig {
    #[config(env = "APP_PORT", default = "8080")]
    port: u16,

    // Automatically reads DB_HOST, DB_PORT
    #[config(nested)]
    db: DatabaseConfig,

    // Parses comma-separated strings like "80, 443, 8080"
    #[config(default = "80,443")]
    allowed_ports: Vec<u16>,

    // Optional fields return None if missing
    secret_key: Option<String>,
}

fn main() {
    // Loads from .env file and system environment variables
    let config = AppConfig::load();
    println!("{:#?}", config);
}
```

---

## 🛠️ Macro Attributes

| Attribute | Description | Example |
| :--- | :--- | :--- |
| `#[config(env = "...")]` | Custom environment variable key name | `#[config(env = "CUSTOM_KEY")]` |
| `#[config(default = "...")]` | Default string value if variable is missing | `#[config(default = "127.0.0.1")]` |
| `#[config(nested)]` | Nest another `Config` struct with prefixing | `#[config(nested)]` |
| `#[config(prefix = "...")]` | Override nested struct prefix | `#[config(prefix = "CUSTOM_PREFIX_")]` |

---

## 💻 CLI Tool

`smart-env` comes with an integrated CLI tool to inspect and validate your environment configurations.

### Installation

```bash
cargo install smart-env-cli
```

### Commands

* **Validate `.env` syntax:**
  ```bash
  smart-env check
  ```

* **Inspect & color-dump current values with their source (`system` vs `.env`):**
  ```bash
  smart-env dump
  ```

---

## 🧪 Error Handling & Testing

If you prefer to avoid panics on missing or invalid configuration variables, use `try_load()` or `try_load_from_vars()`:

```rust
fn main() -> Result<(), smart_env::EnvError> {
    let config = AppConfig::try_load()?;
    Ok(())
}

#[test]
fn test_config_loading() {
    let test_vars = vec![
        ("APP_PORT", "9000"),
        ("DB_HOST", "127.0.0.1"),
    ];

    let config = AppConfig::load_from_vars(test_vars);
    assert_eq!(config.port, 9000);
}
```

---

## 📄 License

Dual-licensed under [MIT](LICENSE-MIT) or [Apache-2.0](LICENSE-APACHE).