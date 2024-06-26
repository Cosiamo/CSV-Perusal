use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use error::{CSVPerusalError, CellError};
use serde::{Serialize, Deserialize};
use csv::{ByteRecord, ReaderBuilder};
use open::{assign_byterecord, csv_read};
use core::fmt;

pub mod open;
pub mod dates;
pub mod regex;
pub mod numbers;
pub mod error;
pub mod types;
pub mod vector_to_byterecord;

/// `CSVType` is the organized the data from [`csv::ByteRecord`]. 
/// It's meant to make the generic output data from the [`csv`] package easier to work with by giving each cell a specific data type. 
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
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
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
///        row.iter().for_each(|cell|{
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
    let data = csv_read(path)?; 
    assign_byterecord(data)
}

pub trait ParseSpreadSheet {
    fn get_csv_types(self) -> Result<Vec<Vec<CSVType>>, CSVPerusalError>;
}

impl ParseSpreadSheet for Vec<ByteRecord> {
    fn get_csv_types(self) -> Result<Vec<Vec<CSVType>>, CSVPerusalError> {
        assign_byterecord(self)
    }
}

impl ParseSpreadSheet for Vec<Vec<String>> {
    fn get_csv_types(self) -> Result<Vec<Vec<CSVType>>, CSVPerusalError> {
        let byterecord = vector_to_byterecord!(self);
        assign_byterecord(byterecord)
    }
}

impl ParseSpreadSheet for Vec<Vec<&str>> {
    fn get_csv_types(self) -> Result<Vec<Vec<CSVType>>, CSVPerusalError> {
        let byterecord = vector_to_byterecord!(self);
        assign_byterecord(byterecord)
    }
}