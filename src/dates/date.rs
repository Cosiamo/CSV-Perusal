use chrono::NaiveDate;

use crate::{ByteString, CSVType};

impl ByteString {
    pub fn date_match(&self) -> CSVType {
        let date = &self.s.replace("/", "-").replace(".", "-").trim().to_string();
        match date.len() {
            10 => match date {
                // mm/dd/yyyy
                d if &d[2..3] == "-" && &d[5..6] == "-"
                => match NaiveDate::parse_from_str(d, "%m-%d-%Y") {
                    Ok(date) => return CSVType::Date(date),
                    // dd/mm/yyyy
                    Err(_) => match NaiveDate::parse_from_str(d, "%d-%m-%Y") {
                        Ok(date) => return CSVType::Date(date),
                        Err(_) => return self.convert_to_string(),
                    },
                },
                // yyyy/mm/dd
                d if &d[4..5] == "-" && &d[7..8] == "-"
                => match NaiveDate::parse_from_str(d, "%Y-%m-%d") {
                    Ok(date) => return CSVType::Date(date),
                    // yyyy/dd/mm
                    Err(_) => match NaiveDate::parse_from_str(d, "%Y-%d-%m") {
                        Ok(date) => return CSVType::Date(date),
                        Err(_) => return self.convert_to_string(),
                    },
                },
                _ => return self.convert_to_string(),
            },
            9 => match date {
                // m/dd/yyyy
                d if &d[1..2] == "-" && &d[4..5] == "-" 
                => match NaiveDate::parse_from_str(d, "%m-%d-%Y") {
                    Ok(date) => return CSVType::Date(date),
                    Err(_) => return self.convert_to_string(),
                },
                // mm/d/yyyy
                d if &d[2..3] == "-" && &d[4..5] == "-" 
                => match NaiveDate::parse_from_str(d, "%m-%d-%Y") {
                    Ok(date) => return CSVType::Date(date),
                    // dd/m/yyyy
                    Err(_) => match NaiveDate::parse_from_str(d, "%d-%m-%Y") {
                        Ok(date) => return CSVType::Date(date),
                        Err(_) => return self.convert_to_string(),
                    },
                },
                // yyyy/mm/d
                d if &d[4..5] == "-" && &d[7..8] == "-" 
                => match NaiveDate::parse_from_str(d, "%Y-%m-%d") {
                    Ok(date) => return CSVType::Date(date),
                    // yyyy/dd/m
                    Err(_) => match NaiveDate::parse_from_str(d, "%Y-%d-%m") {
                        Ok(date) => return CSVType::Date(date),
                        Err(_) => return self.convert_to_string(),
                    },
                },
                // yyyy/m/dd
                d if &d[4..5] == "-" && &d[6..7] == "-" 
                => match NaiveDate::parse_from_str(d, "%Y-%m-%d") {
                    Ok(date) => return CSVType::Date(date),
                    Err(_) => return self.convert_to_string(),
                },
                _ => return self.convert_to_string(),
            },
            8 => match date {
                // m/d/yyyy
                d if &d[1..2] == "-" && &d[3..4] == "-" 
                => match NaiveDate::parse_from_str(d, "%m-%d-%Y") {
                    Ok(date) => return CSVType::Date(date),
                    Err(_) => return self.convert_to_string(),
                },
                // yyyy/m/d
                d if &d[4..5] == "-" && &d[6..7] == "-" 
                => match NaiveDate::parse_from_str(d, "%Y-%m-%d") {
                    Ok(date) => return CSVType::Date(date),
                    Err(_) => return self.convert_to_string(),
                },
                // mm/dd/yy
                d if &d[2..3] == "-" && &d[5..6] == "-"
                => match NaiveDate::parse_from_str(d, "%m-%d-%y") {
                    Ok(date) => return CSVType::Date(date),
                    // dd/mm/yy
                    Err(_) => match NaiveDate::parse_from_str(d, "%d-%m-%y") {
                        Ok(date) => return CSVType::Date(date),
                        Err(_) => return self.convert_to_string(),
                    },
                },
                _ => return self.convert_to_string(),
            },
            7 => match date {
                // m/dd/yy
                d if &d[1..2] == "-" && &d[4..5] == "-"
                => match NaiveDate::parse_from_str(d, "%m-%d-%y") {
                    Ok(date) => return CSVType::Date(date),
                    Err(_) => return self.convert_to_string(),
                },
                // mm/d/yy
                d if &d[2..3] == "-" && &d[4..5] == "-"
                => match NaiveDate::parse_from_str(d, "%m-%d-%y") {
                    Ok(date) => return CSVType::Date(date),
                    // dd/m/yy
                    Err(_) => match NaiveDate::parse_from_str(d, "%d-%m-%y") {
                        Ok(date) => return CSVType::Date(date),
                        // yy/m/dd
                        Err(_) => match NaiveDate::parse_from_str(d, "%y-%m-%d") {
                            Ok(date) => return CSVType::Date(date),
                            Err(_) => return self.convert_to_string(),
                        },
                    },
                },
                // yy/mm/d
                d if &d[2..3] == "-" && &d[5..6] == "-"
                => match NaiveDate::parse_from_str(d, "%y-%m-%d") {
                    Ok(date) => return CSVType::Date(date),
                    // yy/dd/m
                    Err(_) => match NaiveDate::parse_from_str(d, "%y-%d-%m") {
                        Ok(date) => return CSVType::Date(date),
                        Err(_) => return self.convert_to_string(),
                    },
                },
                _ => return self.convert_to_string(),
            },
            6 => match date {
                // m/d/yy
                d if &d[1..2] == "-" && &d[3..4] == "-"
                => match NaiveDate::parse_from_str(d, "%m-%d-%y") {
                    Ok(date) => return CSVType::Date(date),
                    Err(_) => return self.convert_to_string(),
                },
                // yy/m/d
                d if &d[2..3] == "-" && &d[4..5] == "-"
                => match NaiveDate::parse_from_str(d, "%y-%m-%d") {
                    Ok(date) => return CSVType::Date(date),
                    Err(_) => return self.convert_to_string(),
                },
                _ => return self.convert_to_string(),
            },
            _ => return self.convert_to_string(),
        }
    }   

    pub fn date_w_abbrv_match(&self) -> CSVType {
        let date = &self.s.replace("/", "-").replace(".", "-").trim().to_string();
        match date.len() {
            11 => match date {
                // dd/mmm/yyyy
                dt if &dt[2..3] == "-" && &dt[6..7] == "-" 
                => match NaiveDate::parse_from_str(dt, "%d-%b-%Y") {
                    Ok(date) => return CSVType::Date(date),
                    Err(_) => return self.convert_to_string(),
                },
                _ => return self.convert_to_string(),
            },
            9 => match date {
                // dd/mmm/yy
                dt if &dt[2..3] == "-" && &dt[6..7] == "-" 
                => match NaiveDate::parse_from_str(dt, "%d-%b-%y") {
                    Ok(date) => return CSVType::Date(date),
                    Err(_) => return self.convert_to_string(),
                },
                _ => return self.convert_to_string(),
            },
            _ => return self.convert_to_string(),
        }
    }
}