use chrono::NaiveTime;

use crate::{ByteString, CSVType};

impl ByteString {
    pub fn time_match(&self) -> CSVType {
        let time = &self.s.trim().to_string();
        match time {
            t if t.to_ascii_uppercase().contains("AM") || t.to_ascii_uppercase().contains("PM") 
            => match t {
                // hh:mm AM && h:mm AM
                t if t.len() < 9
                => match NaiveTime::parse_from_str(&t.to_ascii_uppercase(), "%I:%M %p") {
                    Ok(time) => return CSVType::Time(time),
                    Err(_) => return self.convert_to_string(),
                },
                // hh:mm:ss AM && h:mm:ss AM
                _ => match NaiveTime::parse_from_str(&t.to_ascii_uppercase(), "%I:%M:%S %p") {
                    Ok(time) => return CSVType::Time(time),
                    Err(_) => return self.convert_to_string(),
                },
            },
            t =>  match t {
                // hh:mm && h:mm
                t if t.len() < 6
                => match NaiveTime::parse_from_str(&*self.s, "%H:%M") {
                    Ok(time) => return CSVType::Time(time),
                    Err(_) => return self.convert_to_string(),
                },
                // hh:mm:ss && h:mm:ss
                _ => match NaiveTime::parse_from_str(&*self.s, "%H:%M:%S") {
                    Ok(time) => return CSVType::Time(time),
                    Err(_) => return self.convert_to_string(),
                },
            },
        }
    }
}