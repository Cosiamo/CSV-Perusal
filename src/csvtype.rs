use core::fmt;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, PartialOrd)]
pub enum CSVType {
    Int(i64),
    Float(f64),
    String(String),
    Date(String),
    Time(String),
    DateTime(String),
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