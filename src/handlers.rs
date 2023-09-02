use std::path::PathBuf;

use thiserror::Error;

use crate::cli::Cli;

pub mod dotparty;
//pub mod e621;

#[derive(Debug, Error)]
pub enum HandlerError {
    #[error("Failed to read the directory {0}")]
    FailedToReadDir(String),
    #[error("Failed to parse the filename {0}")]
    InvalidFilename(String),
    #[error("Failed to create the directory {0}")]
    FailedToCreateDir(String),
    #[error("Failed to move file {0} to {1}")]
    FailedToMoveDir(String, String),
}

pub trait HandlerImpl {
    fn new(path: &PathBuf, update: Option<String>) -> Self;
    fn handle(&self) -> Result<(), HandlerError>;
    fn update(&self) -> Result<(), HandlerError>;
}
