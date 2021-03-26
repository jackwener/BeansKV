mod database;
mod datafile;
mod utils;
mod keydir;
mod error;
mod hintfile;
mod config;

pub use database::Database;
pub use database::Options;

pub use database::new;
pub use database::*;

pub type ErrorResult<T> = Result<T, Box<dyn std::error::Error>>;

