use chrono::NaiveDate;

use crate::{types::ByteString, CSVType};

impl ByteString {
    pub fn date_match(&self) -> Option<CSVType> {
        let fmt_date = &self.bytestring
            .replace("/", "-")
            .replace(".", "-")
            .trim()
            .to_string();

        // mm/dd/yyyy
        if let Ok(date) = NaiveDate::parse_from_str(fmt_date, "%m-%d-%Y") {
            return Some(CSVType::Date(date))
        }
        // dd/mm/yyyy
        if let Ok(date) = NaiveDate::parse_from_str(fmt_date, "%d-%m-%Y") {
            return Some(CSVType::Date(date))
        }
        // yyyy/mm/dd
        if let Ok(date) = NaiveDate::parse_from_str(fmt_date, "%Y-%m-%d") {
            return Some(CSVType::Date(date))
        }
        // yyyy/dd/mm
        if let Ok(date) = NaiveDate::parse_from_str(fmt_date, "%Y-%d-%m") {
            return Some(CSVType::Date(date))
        }
        
        // mm/dd/yy
        if let Ok(date) = NaiveDate::parse_from_str(fmt_date, "%m-%d-%y") {
            return Some(CSVType::Date(date))
        }
        // dd/mm/yy
        if let Ok(date) = NaiveDate::parse_from_str(fmt_date, "%d-%m-%y") {
            return Some(CSVType::Date(date))
        }
        // yy/mm/dd
        if let Ok(date) = NaiveDate::parse_from_str(fmt_date, "%y-%m-%d") {
            return Some(CSVType::Date(date))
        }
        // yy/dd/mm
        if let Ok(date) = NaiveDate::parse_from_str(fmt_date, "%y-%d-%m") {
            return Some(CSVType::Date(date))
        }

        return None;
    }   

    pub fn date_w_abbrv_match(&self) -> CSVType {
        let string = &self.bytestring.replace("/", "-").replace(".", "-").trim().to_string();
        match string.len() {
            11 => match string {
                // dd/mmm/yyyy
                string_to_date if &string_to_date[2..3] == "-" && &string_to_date[6..7] == "-" 
                => match NaiveDate::parse_from_str(string_to_date, "%d-%b-%Y") {
                    Ok(date) => return CSVType::Date(date),
                    Err(_) => return self.convert_to_string(),
                },
                _ => return self.convert_to_string(),
            },
            9 => match string {
                // dd/mmm/yy
                string_to_date if &string_to_date[2..3] == "-" && &string_to_date[6..7] == "-" 
                => match NaiveDate::parse_from_str(string_to_date, "%d-%b-%y") {
                    Ok(date) => return CSVType::Date(date),
                    Err(_) => return self.convert_to_string(),
                },
                _ => return self.convert_to_string(),
            },
            _ => return self.convert_to_string(),
        }
    }
}