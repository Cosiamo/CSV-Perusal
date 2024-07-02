use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use error::{CSVPerusalError, CellError};
use serde::{Serialize, Deserialize};
use csv::{ByteRecord, ReaderBuilder};
use open::{assign_byterecord, csv_read};
use core::fmt;

pub mod error;
pub mod vector_to_byterecord;
pub(crate) mod open;
pub(crate) mod dates;
pub(crate) mod regex;
pub(crate) mod numbers;
pub(crate) mod types;

/// `CSVType` is the organized the data from [`csv::ByteRecord`]. 
/// It's meant to make the generic output data from the [`csv`] package easier to work with by giving each cell a specific data type. 
/// Date, Time, and DateTime use [`chrono`]'s [`chrono::NaiveDate`], [`chrono::NaiveTime`], and [`chrono::NaiveDateTime`] respectively.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum CSVType {
    Int(i64),
    Float(f64),
    String(String),
    Date(NaiveDate),
    Time(NaiveTime),
    DateTime(NaiveDateTime),
    Error(CellError),
    Empty,
}

impl fmt::Display for CSVType {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, fmt)
    }
}

/// Input the file path to a CSV file you want parsed. 
/// It will return a two-dimensional vector of [`CSVType`].
/// 
/// # Example
/// 
/// ```rust
/// use csv_perusal::{open_csv, CSVType};
///
/// fn main() {
///     let path = "test_data/MOCK_DATA.csv";
///     let data = open_csv(path).unwrap();
/// 
///     data.iter().for_each(|row| {
///        row.iter().for_each(|cell| {
///            match cell {
///                CSVType::Int(val) => print!("INT:{:?},", val),
///                CSVType::Float(val) => print!("FLOAT:{:?},", val),
///                CSVType::String(val) => print!("STRING:{:?},", val),
///                CSVType::Date(val) => print!("DATE:{:?},", val),
///                CSVType::Time(val) => print!("TIME:{:?},", val),
///                CSVType::DateTime(val) => print!("DATETIME:{:?},", val),
///                CSVType::Error(e) => print!("ERROR:{:?},", e),
///                CSVType::Empty => print!("NONE,"),
///            }
///        });
///        print!("\n");
///     );
/// }
/// ``` 
pub fn open_csv(path: &str) -> Result<Vec<Vec<CSVType>>, CSVPerusalError> {
    let byte_records = csv_read(path)?; 
    byte_records.get_csv_types()
}

pub trait ParseSeparatedValues {
    fn get_csv_types(self) -> Result<Vec<Vec<CSVType>>, CSVPerusalError>;
}

impl ParseSeparatedValues for Vec<ByteRecord> {
    /// Organizes [`csv::ByteRecord`] into specific data types via the [`CSVType`] struct.
    fn get_csv_types(self) -> Result<Vec<Vec<CSVType>>, CSVPerusalError> {
        assign_byterecord(self)
    }
}

impl ParseSeparatedValues for Vec<Vec<String>> {
    /// Organizes two-dimensional [`String`] vector into specific data types via the [`CSVType`] struct.
    fn get_csv_types(self) -> Result<Vec<Vec<CSVType>>, CSVPerusalError> {
        let byte_records = vector_to_byterecord!(self);
        assign_byterecord(byte_records)
    }
}

impl ParseSeparatedValues for Vec<Vec<&str>> {
    /// Organizes two-dimensional [`str`] vector into specific data types via the [`CSVType`] struct.
    fn get_csv_types(self) -> Result<Vec<Vec<CSVType>>, CSVPerusalError> {
        let byte_records = vector_to_byterecord!(self);
        assign_byterecord(byte_records)
    }
}