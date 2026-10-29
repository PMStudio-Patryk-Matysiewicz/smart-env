use smart_env_core::ConfigBuilder;
use std::env;
use std::path::Path;
use smart_env_core::{Source, explain_all};


const RESET: &str = "\x1b[0m";
const STANDARD: &str = "\x1b[0m";
const CYAN: &str = "\x1b[36m";
const GREEN: &str = "\x1b[32m";
const RED: &str = "\x1b[31m";
const YELLOW: &str = "\x1b[33m"; 

fn main() {
    let args: Vec<String> = env::args().collect();
    let command = args.get(1).map(|s| s.as_str()).unwrap_or("help");

    match command {
        "check" => {
            let env_path = ".env";
            if !Path::new(env_path).exists() {
                println!("[smart-env] Warning: .env file not found in current directory.");
                return;
            }

            let store = ConfigBuilder::new().load_env_file(env_path).build();
            println!(
                "[smart-env] SUCCESS: File '{}' loaded successfully. Total variables: {}",
                env_path,
                store.len()
            );
        }
        "dump" => {
            println!("{}[smart-env]{} Environment Variables:", STANDARD, RESET);
            
            for exp in explain_all() {
                if let Some(val) = exp.value {
                    let (src_tag, src_color) = match exp.source {
                        Source::System => ("system", CYAN),
                        Source::EnvFile => (".env  ", GREEN),
                        Source::Missing => ("missing", RED),
                    };

                    println!(
                        "  {}{}{}\t=> {}{}{}{}={}",
                        src_color, src_tag, RESET,
                        YELLOW, exp.key, RESET,
                        STANDARD, val,
                    );
                }
            }
        }
        _ => {
            println!("smart-env CLI v0.1.0");
            println!("Usage:");
            println!("  smart-env check   Validate .env file syntax");
            println!("  smart-env dump    Print loaded environment variables");
        }
    }
}