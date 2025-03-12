use chrono::NaiveTime;

use crate::{types::ByteString, CSVType};

impl ByteString {
    pub fn time_match(&self) -> Option<CSVType> {
        let fmt_time = &self.bytestring.trim().to_ascii_uppercase();
        
        // hh:mm AM && h:mm AM
        if let Ok(time) =  NaiveTime::parse_from_str(&fmt_time, "%I:%M %p") {
            return Some(CSVType::Time(time))
        }
        // hh:mm:ss AM && h:mm:ss AM
        if let Ok(time) =  NaiveTime::parse_from_str(&fmt_time, "%I:%M:%S %p") {
            return Some(CSVType::Time(time))
        }
        // hh:mm && h:mm
        if let Ok(time) =  NaiveTime::parse_from_str(&fmt_time, "%H:%M") {
            return Some(CSVType::Time(time))
        }
        // hh:mm:ss && h:mm:ss
        if let Ok(time) =  NaiveTime::parse_from_str(&fmt_time, "%H:%M:%S") {
            return Some(CSVType::Time(time))
        }
        return None
    }
}