use chrono::NaiveTime;

use crate::{types::ByteString, CSVType};

impl ByteString {
    pub fn time_match(&self) -> CSVType {
        let string = &self.bytestring.trim().to_string();
        match string {
            am_pm if am_pm.to_ascii_uppercase().contains("AM") || am_pm.to_ascii_uppercase().contains("PM") 
            => match am_pm {
                // hh:mm AM && h:mm AM
                string_to_time if string_to_time.len() < 9
                => match NaiveTime::parse_from_str(&string_to_time.to_ascii_uppercase(), "%I:%M %p") {
                    Ok(time) => return CSVType::Time(time),
                    Err(_) => return self.convert_to_string(),
                },
                // hh:mm:ss AM && h:mm:ss AM
                _ => match NaiveTime::parse_from_str(&am_pm.to_ascii_uppercase(), "%I:%M:%S %p") {
                    Ok(time) => return CSVType::Time(time),
                    Err(_) => return self.convert_to_string(),
                },
            },
            string_to_time =>  match string_to_time {
                // hh:mm && h:mm
                string_to_time if string_to_time.len() < 6
                => match NaiveTime::parse_from_str(&*self.bytestring, "%H:%M") {
                    Ok(time) => return CSVType::Time(time),
                    Err(_) => return self.convert_to_string(),
                },
                // hh:mm:ss && h:mm:ss
                _ => match NaiveTime::parse_from_str(&*self.bytestring, "%H:%M:%S") {
                    Ok(time) => return CSVType::Time(time),
                    Err(_) => return self.convert_to_string(),
                },
            },
        }
    }
}