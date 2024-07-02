use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Errors from opening a file.
#[derive(Error, Debug)]
pub enum CSVPerusalError {
    #[error("Error processing the CSV file: `{0}`")]
    FileError(#[from] csv::Error),
    #[error("`{0}`")]
    IOError(std::io::Error),
}

// Error from parsing a value from a CSV file.
#[derive(Error, Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub enum CellError {
    #[error("A cell could not be parsed into an int, float, date, or string")]
    DataParseError
}