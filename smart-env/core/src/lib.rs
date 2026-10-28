mod builder;
mod env_file;
mod error;
mod explain;
mod get;
pub mod parser;
mod state;
mod store;

pub use builder::ConfigBuilder;
pub use error::EnvError;
pub use explain::{explain, Explanation, Source};
pub use get::{get, get_or};
pub use store::EnvStore;

pub fn init() {
    state::load_env_file();
}
