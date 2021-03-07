mod kv;
mod datafile;
mod util;
mod keydir;
mod error;
mod hintfile;

pub type ErrorResult<T> = Result<T, Box<dyn std::error::Error>>;