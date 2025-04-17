pub mod dto;
mod error;
pub mod homepage;
pub mod ratel;

pub use error::*;

pub type Result<T> = std::result::Result<T, error::Error>;
