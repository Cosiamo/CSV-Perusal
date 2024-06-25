use chrono::{NaiveDate, NaiveDateTime, NaiveTime};
use serde::{Serialize, Deserialize};
use csv::{ByteRecord, ReaderBuilder};
use open::assign_byterecord;use core::fmt;

pub mod open;
pub mod dates;
pub mod regex;
pub mod numbers;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum CSVType {
    Int(i64),
    Float(f64),
    String(String),
    Date(NaiveDate),
    Time(NaiveTime),
    DateTime(NaiveDateTime),
    Error(String),
    Empty,
}

impl fmt::Display for CSVType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

pub struct ByteString {
    pub(crate) s: String,
}

pub struct Byte<'slice> {
    pub(crate) b: &'slice [u8],
}

pub trait ParseSpreadSheet {
    fn get_csv_types(self) -> Result<Vec<Vec<CSVType>>, csv::Error>;
}

impl ParseSpreadSheet for Vec<ByteRecord> {
    fn get_csv_types(self) -> Result<Vec<Vec<CSVType>>, csv::Error> {
        assign_byterecord(self)
    }
}

macro_rules! vectors_to_byterecord {
    ($data:expr) => {{
        let tabbed = $data.iter().map(|row| row.join("\t")).collect::<Vec<String>>();
        let tsv = tabbed.join("\n");
        let mut reader = ReaderBuilder::new().delimiter(b'\t').from_reader(tsv.as_bytes());
        let mut byterecord = Vec::new(); 
        for result in reader.byte_records() { byterecord.push(result?) }
        assign_byterecord(byterecord)
    }};
}

impl ParseSpreadSheet for Vec<Vec<String>> {
    fn get_csv_types(self) -> Result<Vec<Vec<CSVType>>, csv::Error> {
        vectors_to_byterecord!(self)
    }
}

impl ParseSpreadSheet for Vec<Vec<&str>> {
    fn get_csv_types(self) -> Result<Vec<Vec<CSVType>>, csv::Error> {
        vectors_to_byterecord!(self)
    }
}