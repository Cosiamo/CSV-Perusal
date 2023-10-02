use crate::csvtype::{CSVType, ByteString};
use chrono::prelude::*;

impl ByteString {

    pub fn date_match(&self) -> CSVType {
        let date = &self.s.replace("/", "-").trim().to_string();
        match date.len() {
            10 => match date {
                // mm/dd/yyyy
                d if &d[2..3] == "-" && &d[5..6] == "-"
                => match NaiveDate::parse_from_str(d, "%m-%d-%Y") {
                    Ok(date) => {
                        return CSVType::Date(date.to_string())
                    },
                    Err(_) => match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    },
                }
                // yyyy/mm/dd
                d if &d[4..5] == "-" && &d[7..8] == "-"
                => match NaiveDate::parse_from_str(d, "%Y-%m-%d") {
                    Ok(date) => return CSVType::Date(date.to_string()),
                    Err(_) => match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    },
                } 
                _ => match self.s.parse::<String>() {
                    Ok(s) => return CSVType::String(s),
                    Err(e) => return CSVType::Error(e),
                }
            },
            9 => match date {
                // m/dd/yyyy
                d if &d[1..2] == "-" && &d[4..5] == "-" 
                => match NaiveDate::parse_from_str(d, "%m-%d-%Y") {
                    Ok(date) => return CSVType::Date(date.to_string()),
                    Err(_) => match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    },
                },
                // mm/d/yyyy
                d if &d[2..3] == "-" && &d[4..5] == "-" 
                => match NaiveDate::parse_from_str(d, "%m-%d-%Y") {
                    Ok(date) => return CSVType::Date(date.to_string()),
                    Err(_) => match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    },
                },
                // yyyy/mm/d
                d if &d[4..5] == "-" && &d[7..8] == "-" 
                => match NaiveDate::parse_from_str(d, "%Y-%m-%d") {
                    Ok(date) => return CSVType::Date(date.to_string()),
                    Err(_) => match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    },
                },
                // yyyy/m/dd
                d if &d[4..5] == "-" && &d[6..7] == "-" 
                => match NaiveDate::parse_from_str(d, "%Y-%m-%d") {
                    Ok(date) => return CSVType::Date(date.to_string()),
                    Err(_) => match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    },
                } 
                _ => match self.s.parse::<String>() {
                    Ok(s) => return CSVType::String(s),
                    Err(e) => return CSVType::Error(e),
                },
            },
            8 => match date {
                // m/d/yyyy
                d if &d[1..2] == "-" && &d[3..4] == "-" 
                => match NaiveDate::parse_from_str(&*self.s.replace("/", "-"), "%m-%d-%Y") {
                    Ok(date) => return CSVType::Date(date.to_string()),
                    Err(_) => match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    },
                },
                // yyyy/m/d
                d if &d[4..5] == "-" && &d[6..7] == "-" 
                => match NaiveDate::parse_from_str(&*self.s.replace("/", "-"), "%Y-%m-%d") {
                    Ok(date) => return CSVType::Date(date.to_string()),
                    Err(_) => match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    },
                },
                // mm/dd/yy
                d if &d[2..3] == "-" && &d[5..6] == "-"
                => match NaiveDate::parse_from_str(&*self.s.replace("/", "-"), "%m-%d-%y") {
                    Ok(date) => return CSVType::Date(date.to_string()),
                    Err(_) => match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    },
                },
                _ => match self.s.parse::<String>() {
                    Ok(s) => return CSVType::String(s),
                    Err(e) => return CSVType::Error(e),
                },
            },
            // 7 => {
            //     // m/dd/yy
            //     if &d[1..2] == "/" && &d[4..5] == "/" 
            //     || &d[1..2] == "-" && &d[4..5] == "-" {
            //         return CSVType::Date((*self.s).to_string());
            //     }
            //     // mm/d/yy
            //     else if &d[2..3] == "/" && &d[4..5] == "/" 
            //     || &d[2..3] == "-" && &d[4..5] == "-" {
            //         return CSVType::Date((*self.s).to_string());
            //     } else {
            //         match self.s.parse::<String>() {
            //             Ok(s) => return CSVType::String(s),
            //             Err(e) => return CSVType::Error(e),
            //         };
            //     }
            // },
            // 6 => {
            //     // m/d/yy
            //     if &d[1..2] == "/" && &d[3..4] == "/" 
            //     || &d[1..2] == "-" && &d[3..4] == "-" {
            //         return CSVType::Date((*self.s).to_string());
            //     }
            //     // yy/d/m
            //     else if &d[2..3] == "/" && &d[4..5] == "/" 
            //     || &d[2..3] == "-" && &d[4..5] == "-" {
            //         return CSVType::Date((*self.s).to_string());
            //     } else {
            //         match self.s.parse::<String>() {
            //             Ok(s) => return CSVType::String(s),
            //             Err(e) => return CSVType::Error(e),
            //         };
            //     }
            // },
            _ => match self.s.parse::<String>() {
                Ok(s) => return CSVType::String(s),
                Err(e) => return CSVType::Error(e),
            },
        }
    }   

    pub fn time_match(&self) -> CSVType {
        let time = &self.s.trim().to_string();
        match time {
            t if t.to_uppercase().contains("AM") 
            => match t {
                tam if tam.len() < 10
                => match tam {
                    // hh:mm AM 
                    tam if &tam[2..3] == ":" 
                    => match NaiveTime::parse_from_str(&*self.s, "%I:%M %P") {
                        Ok(time) => return CSVType::Time(time.to_string()),
                        Err(_) => match self.s.parse::<String>() {
                            Ok(s) => return CSVType::String(s),
                            Err(e) => return CSVType::Error(e),
                        },
                    },
                    // h:mm AM
                    tam if &tam[1..2] == ":" 
                    => match NaiveTime::parse_from_str(&*self.s, "%I:%M %P") {
                        Ok(time) => return CSVType::Time(time.to_string()),
                        Err(_) => match self.s.parse::<String>() {
                            Ok(s) => return CSVType::String(s),
                            Err(e) => return CSVType::Error(e),
                        },
                    },
                    _ => match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    },
                },
                // hh:mm:ss AM
                tam if &tam[2..3] == ":" && &tam[5..6] == ":" 
                => match NaiveTime::parse_from_str(&*self.s, "%I:%M:%S %P") {
                    Ok(time) => return CSVType::Time(time.to_string()),
                    Err(_) => match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    },
                },
                // h:mm:ss AM
                tam if &tam[1..2] == ":" && &tam[4..5] == ":"
                => match NaiveTime::parse_from_str(&*self.s, "%I:%M:%S %P") {
                    Ok(time) => return CSVType::Time(time.to_string()),
                    Err(_) => match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    },
                }
                _ => match self.s.parse::<String>() {
                    Ok(s) => return CSVType::String(s),
                    Err(e) => return CSVType::Error(e),
                },
            },
            t if t.to_uppercase().contains("PM") 
            => match t {
                tpm if tpm.len() < 10
                => match tpm {
                    // hh:mm PM 
                    tpm if &tpm[2..3] == ":" 
                    => match NaiveTime::parse_from_str(&*self.s, "%I:%M %p") {
                        Ok(time) => return CSVType::Time(time.to_string()),
                        Err(_) => match self.s.parse::<String>() {
                            Ok(s) => return CSVType::String(s),
                            Err(e) => return CSVType::Error(e),
                        },
                    },
                    // h:mm PM
                    tpm if &tpm[1..2] == ":" 
                    => match NaiveTime::parse_from_str(&*self.s, "%I:%M %p") {
                        Ok(time) => return CSVType::Time(time.to_string()),
                        Err(_) => match self.s.parse::<String>() {
                            Ok(s) => return CSVType::String(s),
                            Err(e) => return CSVType::Error(e),
                        },
                    },
                    _ => match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    },
                },
                // hh:mm:ss PM
                tpm if &tpm[2..3] == ":" && &tpm[5..6] == ":" 
                => match NaiveTime::parse_from_str(&*self.s, "%I:%M:%S %p") {
                    Ok(time) => return CSVType::Time(time.to_string()),
                    Err(_) => match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    },
                },
                // h:mm:ss PM
                tpm if &tpm[1..2] == ":" && &tpm[4..5] == ":"
                => match NaiveTime::parse_from_str(&*self.s, "%I:%M:%S %p") {
                    Ok(time) => return CSVType::Time(time.to_string()),
                    Err(_) => match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    },
                }
                _ => match self.s.parse::<String>() {
                    Ok(s) => return CSVType::String(s),
                    Err(e) => return CSVType::Error(e),
                },
            },
            t =>  match t {
                t if t.len() < 7
                => match t {
                    // hh:mm 
                    t if &t[2..3] == ":" 
                    => match NaiveTime::parse_from_str(&*self.s, "%H:%M") {
                        Ok(time) => return CSVType::Time(time.to_string()),
                        Err(_) => match self.s.parse::<String>() {
                            Ok(s) => return CSVType::String(s),
                            Err(e) => return CSVType::Error(e),
                        },
                    },
                    // h:mm
                    t if &t[1..2] == ":" 
                    => match NaiveTime::parse_from_str(&*self.s, "%H:%M") {
                        Ok(time) => return CSVType::Time(time.to_string()),
                        Err(_) => match self.s.parse::<String>() {
                            Ok(s) => return CSVType::String(s),
                            Err(e) => return CSVType::Error(e),
                        },
                    },
                    _ => match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    },
                }
                // hh:mm:ss
                t if &t[2..3] == ":" && &t[5..6] == ":" 
                => match NaiveTime::parse_from_str(&*self.s, "%H:%M:%S") {
                    Ok(time) => return CSVType::Time(time.to_string()),
                    Err(_) => match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    },
                },
                // h:mm:ss
                t if &t[1..2] == ":" && &t[4..5] == ":"
                => match NaiveTime::parse_from_str(&*self.s, "%H:%M:%S") {
                    Ok(time) => return CSVType::Time(time.to_string()),
                    Err(_) => match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    },
                },
                _ => match self.s.parse::<String>() {
                    Ok(s) => return CSVType::String(s),
                    Err(e) => return CSVType::Error(e),
                },
            },
        }
    }

    pub fn datetime_match(&self) -> CSVType {
        let datetime = &self.s.replace("/", "-").trim().to_string();
        match datetime.len() {
            16 => match datetime {
                // mm/dd/yyyy hh:mm
                dt if &dt[13..14] == ":"
                && &dt[2..3] == "-" && &dt[5..6] == "-" 
                => match NaiveDateTime::parse_from_str(dt, "%m-%d-%Y %H:%M") {
                    Ok(date) => {
                        return CSVType::DateTime(date.to_string())
                    },
                    Err(_) => match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    },
                },
                _ => match self.s.parse::<String>() {
                    Ok(s) => return CSVType::String(s),
                    Err(e) => return CSVType::Error(e),
                }
            },
            15 => match datetime {
                // mm/dd/yyyy h:mm
                dt if &dt[12..13] == ":"
                && &dt[2..3] == "-" && &dt[5..6] == "-" 
                => match NaiveDateTime::parse_from_str(dt, "%m-%d-%Y %H:%M") {
                    Ok(date) => {
                        return CSVType::DateTime(date.to_string())
                    },
                    Err(_) => match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    },
                },
                // m/dd/yyyy hh:mm
                dt if &dt[12..13] == ":"
                && &dt[1..2] == "-" && &dt[4..5] == "-" 
                => match NaiveDateTime::parse_from_str(dt, "%m-%d-%Y %H:%M") {
                    Ok(date) => {
                        return CSVType::DateTime(date.to_string())
                    },
                    Err(_) => match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    },
                },
                // mm/d/yyyy hh:mm
                dt if &dt[12..13] == ":"
                && &dt[2..3] == "-" && &dt[4..5] == "-" 
                => match NaiveDateTime::parse_from_str(dt, "%m-%d-%Y %H:%M") {
                    Ok(date) => {
                        return CSVType::DateTime(date.to_string())
                    },
                    Err(_) => match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    },
                },
                _ => match self.s.parse::<String>() {
                    Ok(s) => return CSVType::String(s),
                    Err(e) => return CSVType::Error(e),
                },
            },
            14 => match datetime {
                // m/dd/yyyy h:mm
                dt if &dt[11..12] == ":"
                && &dt[1..2] == "-" && &dt[4..5] == "-" 
                => match NaiveDateTime::parse_from_str(dt, "%m-%d-%Y %H:%M") {
                    Ok(date) => {
                        return CSVType::DateTime(date.to_string())
                    },
                    Err(_) => match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    },
                },
                // mm/d/yyyy h:mm
                dt if &dt[11..12] == ":"
                && &dt[2..3] == "-" && &dt[4..5] == "-" 
                => match NaiveDateTime::parse_from_str(dt, "%m-%d-%Y %H:%M") {
                    Ok(date) => {
                        return CSVType::DateTime(date.to_string())
                    },
                    Err(_) => match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    },
                },
                // m/d/yyyy hh:mm
                dt if &dt[11..12] == ":"
                && &dt[1..2] == "-" && &dt[3..4] == "-" 
                => match NaiveDateTime::parse_from_str(dt, "%m-%d-%Y %H:%M") {
                    Ok(date) => {
                        return CSVType::DateTime(date.to_string())
                    },
                    Err(_) => match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    },
                },
                // mm/dd/yy hh:mm
                dt if &dt[11..12] == ":"
                && &dt[2..3] == "-" && &dt[5..6] == "-" 
                => match NaiveDateTime::parse_from_str(dt, "%m-%d-%y %H:%M") {
                    Ok(date) => {
                        return CSVType::DateTime(date.to_string())
                    },
                    Err(_) => match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    },
                },
                _ => match self.s.parse::<String>() {
                    Ok(s) => return CSVType::String(s),
                    Err(e) => return CSVType::Error(e),
                },
            },
            13 => match datetime {
                // m/d/yyyy h:mm
                dt if &dt[10..11] == ":"
                && &dt[1..2] == "-" && &dt[3..4] == "-" 
                => match NaiveDateTime::parse_from_str(dt, "%m-%d-%Y %H:%M") {
                    Ok(date) => {
                        return CSVType::DateTime(date.to_string())
                    },
                    Err(_) => match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    },
                },
                // mm/dd/yy h:mm
                dt if &dt[10..11] == ":"
                && &dt[2..3] == "-" && &dt[5..6] == "-" 
                => match NaiveDateTime::parse_from_str(dt, "%m-%d-%y %H:%M") {
                    Ok(date) => {
                        return CSVType::DateTime(date.to_string())
                    },
                    Err(_) => match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    },
                },
                // m/dd/yy hh:mm
                dt if &dt[10..11] == ":"
                && &dt[1..2] == "-" && &dt[4..5] == "-" 
                => match NaiveDateTime::parse_from_str(dt, "%m-%d-%y %H:%M") {
                    Ok(date) => {
                        return CSVType::DateTime(date.to_string())
                    },
                    Err(_) => match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    },
                },
                // mm/d/yy hh:mm
                dt if &dt[10..11] == ":"
                && &dt[2..3] == "-" && &dt[4..5] == "-" 
                => match NaiveDateTime::parse_from_str(dt, "%m-%d-%y %H:%M") {
                    Ok(date) => {
                        return CSVType::DateTime(date.to_string())
                    },
                    Err(_) => match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    },
                },
                _ => match self.s.parse::<String>() {
                    Ok(s) => return CSVType::String(s),
                    Err(e) => return CSVType::Error(e),
                },
            },
            12 => match datetime {
                // m/dd/yy h:mm
                dt if &dt[9..10] == ":"
                && &dt[1..2] == "-" && &dt[4..5] == "-" 
                => match NaiveDateTime::parse_from_str(dt, "%m-%d-%y %H:%M") {
                    Ok(date) => {
                        return CSVType::DateTime(date.to_string())
                    },
                    Err(_) => match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    },
                },
                // mm/d/yy h:mm
                dt if &dt[9..10] == ":"
                && &dt[2..3] == "-" && &dt[4..5] == "-" 
                => match NaiveDateTime::parse_from_str(dt, "%m-%d-%y %H:%M") {
                    Ok(date) => {
                        return CSVType::DateTime(date.to_string())
                    },
                    Err(_) => match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    },
                },
                _ => match self.s.parse::<String>() {
                    Ok(s) => return CSVType::String(s),
                    Err(e) => return CSVType::Error(e),
                },
            },
            _ => match self.s.parse::<String>() {
                Ok(s) => return CSVType::String(s),
                Err(e) => return CSVType::Error(e),
            },
        }
    }

}