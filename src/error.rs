use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CSVPerusalError {
    #[error("Error processing the CSV file: `{0}`")]
    FileError(#[from] csv::Error),
    #[error("`{0}`")]
    IOError(std::io::Error),
}

#[derive(Error, Debug, Serialize, Deserialize, Clone, PartialEq, PartialOrd)]
pub enum CellError {
    #[error("A cell could not be parsed into an int, float, date, or string")]
    DataParseError
}