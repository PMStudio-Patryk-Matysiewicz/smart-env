use smart_env::Config;

#[derive(Config, Debug)]
struct DatabaseConfig {
    #[config(default = "127.0.0.1")]
    host: String,

    #[config(default = "5432")]
    port: u16,
}

#[derive(Config, Debug)]
struct AppConfig {
    #[config(env = "APP_NAME", default = "SmartEnvDemo")]
    name: String,

    #[config(env = "APP_PORT", default = "8080")]
    port: u16,

    // Automatically resolves prefixed variables: DB_HOST, DB_PORT
    #[config(nested)]
    db: DatabaseConfig,

    // Parses comma-separated values into a vector (e.g. "80,443,8080")
    #[config(default = "80,443,8080")]
    ports: Vec<u16>,

    // Optional fields evaluate to None if environment variable is absent
    secret_key: Option<String>,
}

fn main() {
    println!("=== 1. Standard loading (panics on error) ===");
    let config = AppConfig::load();
    println!("{:#?}\n", config);

    println!("=== 2. Safe loading (returns Result) ===");
    match AppConfig::try_load() {
        Ok(safe_config) => println!("Successfully loaded config:\n{:#?}", safe_config),
        Err(err) => eprintln!("Configuration loading failed: {}", err),
    }
}