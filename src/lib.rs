#![doc = include_str!("../README.md")]

use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Serialize, Deserialize};
use csv::{ByteRecord, ReaderBuilder};
use open::{assign_bytes, csv_read};
use errors::{CSVPerusalError, CellError};
use core::fmt;

pub mod errors;
pub mod utils;
pub mod open;
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

/// Input the file path to a CSV file you want parsed and it will return a two-dimensional vector of [`CSVType`].
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
///                CSVType::Error(err) => print!("ERROR:{:?},", err),
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

/// Contains the method for converting grid data to [`CSVType`].
pub trait ParseSeparatedValues {
    /// A required method that converts grid data into a two-dimensional vector of [`CSVType`].
    /// 
    /// # How to Implement
    /// 
    /// ```rust 
    /// impl ParseSeparatedValues for Vec<Vec<MyCustomEnum>> {
    ///     fn get_csv_types(self) -> Result<Vec<Vec<CSVType>>, CSVPerusalError> {
    ///         // Make sure your input datatype can convert to a string.
    ///         // [`utils::grid_to_byterecord!`] will convert your grid data into Vec<[`csv::ByteRecord`]>.
    ///         let byte_records = grid_to_byterecord!(self);
    ///         // Converts Vec<[`csv::ByteRecord`]> to Vec<Vec<[`CSVType`]>>
    ///         assign_bytes(byte_records)
    ///     }
    /// }
    /// ```
    fn get_csv_types(self) -> Result<Vec<Vec<CSVType>>, CSVPerusalError>;
}

impl ParseSeparatedValues for Vec<ByteRecord> {
    /// Organizes [`csv::ByteRecord`] into specific data types via the [`CSVType`] struct.
    fn get_csv_types(self) -> Result<Vec<Vec<CSVType>>, CSVPerusalError> {
        assign_bytes(self)
    }
}

impl ParseSeparatedValues for Vec<Vec<String>> {
    /// Organizes two-dimensional [`String`] vector into specific data types via the [`CSVType`] struct.
    fn get_csv_types(self) -> Result<Vec<Vec<CSVType>>, CSVPerusalError> {
        let byte_records = grid_to_byterecord!(self);
        assign_bytes(byte_records)
    }
}

impl ParseSeparatedValues for Vec<Vec<&str>> {
    /// Organizes two-dimensional [`str`] vector into specific data types via the [`CSVType`] struct.
    fn get_csv_types(self) -> Result<Vec<Vec<CSVType>>, CSVPerusalError> {
        let byte_records = grid_to_byterecord!(self);
        assign_bytes(byte_records)
    }
}