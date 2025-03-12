use chrono::NaiveDateTime;

use crate::{types::ByteString, CSVType};

impl ByteString {
    pub fn datetime_match(&self) -> Option<CSVType> {
        let fmt_dt = &self.bytestring
            .replace("/", "-")
            .replace(".", "-")
            .trim()
            .to_ascii_uppercase();
        // mm/dd/yyyy hh:mm:ss AM
        if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt, "%m-%d-%Y %I:%M:%S %p") {
            return Some(CSVType::DateTime(date_time))
        }
        // dd/mm/yyyy hh:mm:ss AM
        if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt, "%d-%m-%Y %I:%M:%S %p") {
            return Some(CSVType::DateTime(date_time))
        }
        // yyyy/mm/dd hh:mm:ss AM
        if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt, "%Y-%m-%d %I:%M:%S %p") {
            return Some(CSVType::DateTime(date_time))
        }
        // yyyy/dd/mm hh:mm:ss AM
        if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt, "%Y-%d-%m %I:%M:%S %p") {
            return Some(CSVType::DateTime(date_time))
        }
        // mm/dd/yyyy hh:mm:ss
        if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt, "%m-%d-%Y %H:%M:%S") {
            return Some(CSVType::DateTime(date_time))
        }
        // dd/mm/yyyy hh:mm:ss
        if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt, "%d-%m-%Y %H:%M:%S") {
            return Some(CSVType::DateTime(date_time))
        }
        // yyyy/mm/dd hh:mm:ss
        if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt, "%Y-%m-%d %H:%M:%S") {
            return Some(CSVType::DateTime(date_time))
        }
        // mm/dd/yyyy hh:mm AM
        if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt, "%m-%d-%Y %I:%M %p") {
            return Some(CSVType::DateTime(date_time))
        }
        // dd/mm/yyyy hh:mm AM
        if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt, "%d-%m-%Y %I:%M %p") {
            return Some(CSVType::DateTime(date_time))
        }
        // yyyy/mm/dd hh:mm AM
        if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt, "%Y-%m-%d %I:%M %p") {
            return Some(CSVType::DateTime(date_time))
        }
        // yyyy/dd/mm hh:mm AM
        if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt, "%Y-%d-%m %I:%M %p") {
            return Some(CSVType::DateTime(date_time))
        }
        // mm/dd/yy hh:mm:ss AM
        if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt, "%m-%d-%y %I:%M:%S %p") {
            return Some(CSVType::DateTime(date_time))
        }
        // dd/mm/yy hh:mm:ss AM
        if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt, "%d-%m-%y %I:%M:%S %p") {
            return Some(CSVType::DateTime(date_time))
        }
        // yy/mm/dd hh:mm:ss AM
        if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt, "%y-%m-%d %I:%M:%S %p") {
            return Some(CSVType::DateTime(date_time))
        }
        // yy/dd/mm hh:mm:ss AM
        if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt, "%y-%d-%m %I:%M:%S %p") {
            return Some(CSVType::DateTime(date_time))
        }
        // mm/dd/yy hh:mm:ss
        if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt, "%m-%d-%y %H:%M:%S") {
            return Some(CSVType::DateTime(date_time))
        }
        // dd/mm/yy hh:mm:ss
        if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt, "%d-%m-%y %H:%M:%S") {
            return Some(CSVType::DateTime(date_time))
        }
        // yy/mm/dd hh:mm:ss
        if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt, "%y-%m-%d %H:%M:%S") {
            return Some(CSVType::DateTime(date_time))
        }
        // mm/dd/yy hh:mm AM
        if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt, "%m-%d-%y %I:%M %p") {
            return Some(CSVType::DateTime(date_time))
        }
        // dd/mm/yy hh:mm AM
        if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt, "%d-%m-%y %I:%M %p") {
            return Some(CSVType::DateTime(date_time))
        }
        // yy/mm/dd hh:mm AM
        if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt, "%y-%m-%d %I:%M %p") {
            return Some(CSVType::DateTime(date_time))
        }
        // yy/dd/mm hh:mm AM
        if let Ok(date_time) = NaiveDateTime::parse_from_str(&fmt_dt, "%y-%d-%m %I:%M %p") {
            return Some(CSVType::DateTime(date_time))
        }

        return None
    }
}