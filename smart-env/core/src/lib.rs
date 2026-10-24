mod env_file;
mod error;
mod explain;
mod get;
mod parser;
mod state;

pub use error::EnvError;
pub use explain::{explain, Explanation, Source};
pub use get::{get, get_or};

pub fn init() {
    state::load_env_file();
}
