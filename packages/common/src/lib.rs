mod error;
pub mod ratel;
pub mod sys;
pub use error::*;

pub type Result<T> = std::result::Result<T, error::Error>;
