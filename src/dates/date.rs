use chrono::NaiveDate;

use crate::{types::ByteString, CSVType};

impl ByteString {
    pub fn date_match(&self) -> CSVType {
        let string = &self.bytestring
            .replace("/", "-")
            .replace(".", "-")
            .trim()
            .to_string();
        match string.len() {
            10 => match string {
                // mm/dd/yyyy
                string_to_date if &string_to_date[2..3] == "-" && &string_to_date[5..6] == "-"
                => match NaiveDate::parse_from_str(string_to_date, "%m-%d-%Y") {
                    Ok(date) => return CSVType::Date(date),
                    // dd/mm/yyyy
                    Err(_) => match NaiveDate::parse_from_str(string_to_date, "%d-%m-%Y") {
                        Ok(date) => return CSVType::Date(date),
                        Err(_) => return self.convert_to_string(),
                    },
                },
                // yyyy/mm/dd
                string_to_date if &string_to_date[4..5] == "-" && &string_to_date[7..8] == "-"
                => match NaiveDate::parse_from_str(string_to_date, "%Y-%m-%d") {
                    Ok(date) => return CSVType::Date(date),
                    // yyyy/dd/mm
                    Err(_) => match NaiveDate::parse_from_str(string_to_date, "%Y-%d-%m") {
                        Ok(date) => return CSVType::Date(date),
                        Err(_) => return self.convert_to_string(),
                    },
                },
                _ => return self.convert_to_string(),
            },
            9 => match string {
                // m/dd/yyyy
                string_to_date if &string_to_date[1..2] == "-" && &string_to_date[4..5] == "-" 
                => match NaiveDate::parse_from_str(string_to_date, "%m-%d-%Y") {
                    Ok(date) => return CSVType::Date(date),
                    Err(_) => return self.convert_to_string(),
                },
                // mm/d/yyyy
                string_to_date if &string_to_date[2..3] == "-" && &string_to_date[4..5] == "-" 
                => match NaiveDate::parse_from_str(string_to_date, "%m-%d-%Y") {
                    Ok(date) => return CSVType::Date(date),
                    // dd/m/yyyy
                    Err(_) => match NaiveDate::parse_from_str(string_to_date, "%d-%m-%Y") {
                        Ok(date) => return CSVType::Date(date),
                        Err(_) => return self.convert_to_string(),
                    },
                },
                // yyyy/mm/d
                string_to_date if &string_to_date[4..5] == "-" && &string_to_date[7..8] == "-" 
                => match NaiveDate::parse_from_str(string_to_date, "%Y-%m-%d") {
                    Ok(date) => return CSVType::Date(date),
                    // yyyy/dd/m
                    Err(_) => match NaiveDate::parse_from_str(string_to_date, "%Y-%d-%m") {
                        Ok(date) => return CSVType::Date(date),
                        Err(_) => return self.convert_to_string(),
                    },
                },
                // yyyy/m/dd
                string_to_date if &string_to_date[4..5] == "-" && &string_to_date[6..7] == "-" 
                => match NaiveDate::parse_from_str(string_to_date, "%Y-%m-%d") {
                    Ok(date) => return CSVType::Date(date),
                    Err(_) => return self.convert_to_string(),
                },
                _ => return self.convert_to_string(),
            },
            8 => match string {
                // m/d/yyyy
                string_to_date if &string_to_date[1..2] == "-" && &string_to_date[3..4] == "-" 
                => match NaiveDate::parse_from_str(string_to_date, "%m-%d-%Y") {
                    Ok(date) => return CSVType::Date(date),
                    Err(_) => return self.convert_to_string(),
                },
                // yyyy/m/d
                string_to_date if &string_to_date[4..5] == "-" && &string_to_date[6..7] == "-" 
                => match NaiveDate::parse_from_str(string_to_date, "%Y-%m-%d") {
                    Ok(date) => return CSVType::Date(date),
                    Err(_) => return self.convert_to_string(),
                },
                // mm/dd/yy
                string_to_date if &string_to_date[2..3] == "-" && &string_to_date[5..6] == "-"
                => match NaiveDate::parse_from_str(string_to_date, "%m-%d-%y") {
                    Ok(date) => return CSVType::Date(date),
                    // dd/mm/yy
                    Err(_) => match NaiveDate::parse_from_str(string_to_date, "%d-%m-%y") {
                        Ok(date) => return CSVType::Date(date),
                        Err(_) => return self.convert_to_string(),
                    },
                },
                _ => return self.convert_to_string(),
            },
            7 => match string {
                // m/dd/yy
                string_to_date if &string_to_date[1..2] == "-" && &string_to_date[4..5] == "-"
                => match NaiveDate::parse_from_str(string_to_date, "%m-%d-%y") {
                    Ok(date) => return CSVType::Date(date),
                    Err(_) => return self.convert_to_string(),
                },
                // mm/d/yy
                string_to_date if &string_to_date[2..3] == "-" && &string_to_date[4..5] == "-"
                => match NaiveDate::parse_from_str(string_to_date, "%m-%d-%y") {
                    Ok(date) => return CSVType::Date(date),
                    // dd/m/yy
                    Err(_) => match NaiveDate::parse_from_str(string_to_date, "%d-%m-%y") {
                        Ok(date) => return CSVType::Date(date),
                        // yy/m/dd
                        Err(_) => match NaiveDate::parse_from_str(string_to_date, "%y-%m-%d") {
                            Ok(date) => return CSVType::Date(date),
                            Err(_) => return self.convert_to_string(),
                        },
                    },
                },
                // yy/mm/d
                string_to_date if &string_to_date[2..3] == "-" && &string_to_date[5..6] == "-"
                => match NaiveDate::parse_from_str(string_to_date, "%y-%m-%d") {
                    Ok(date) => return CSVType::Date(date),
                    // yy/dd/m
                    Err(_) => match NaiveDate::parse_from_str(string_to_date, "%y-%d-%m") {
                        Ok(date) => return CSVType::Date(date),
                        Err(_) => return self.convert_to_string(),
                    },
                },
                _ => return self.convert_to_string(),
            },
            6 => match string {
                // m/d/yy
                string_to_date if &string_to_date[1..2] == "-" && &string_to_date[3..4] == "-"
                => match NaiveDate::parse_from_str(string_to_date, "%m-%d-%y") {
                    Ok(date) => return CSVType::Date(date),
                    Err(_) => return self.convert_to_string(),
                },
                // yy/m/d
                string_to_date if &string_to_date[2..3] == "-" && &string_to_date[4..5] == "-"
                => match NaiveDate::parse_from_str(string_to_date, "%y-%m-%d") {
                    Ok(date) => return CSVType::Date(date),
                    Err(_) => return self.convert_to_string(),
                },
                _ => return self.convert_to_string(),
            },
            _ => return self.convert_to_string(),
        }
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