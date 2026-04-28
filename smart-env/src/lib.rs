mod env_file;
mod get;
mod parser;
mod error;
mod state;
mod explain;

pub use get::{get, get_or};
pub use explain::{explain, Explanation, Source};
pub use error::EnvError;

pub fn init() {
    state::load_env_file();
}
