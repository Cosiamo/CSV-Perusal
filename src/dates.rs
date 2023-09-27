use crate::csvtype::{CSVType, ByteString};
use chrono::prelude::*;

impl ByteString {

    pub fn date_match(&self) -> CSVType {
        let d = &self.s.trim().to_string();
        match d.len() {
            10 => {
                // mm/dd/yyyy
                if &d[2..3] == "/" && &d[5..6] == "/" 
                || &d[2..3] == "-" && &d[5..6] == "-" {
                    match NaiveDate::parse_from_str(&*self.s.replace("-", "/"), "%m/%d/%Y") {
                        Ok(date) => {
                            return CSVType::Date(date.to_string())
                        },
                        Err(_) => match self.s.parse::<String>() {
                            Ok(s) => return CSVType::String(s),
                            Err(e) => return CSVType::Error(e),
                        },
                    }
                }
                // yyyy/mm/dd
                else if &d[4..5] == "/" && &d[7..8] == "/" 
                || &d[4..5] == "-" && &d[7..8] == "-" {
                    match NaiveDate::parse_from_str(&*self.s.replace("-", "/"), "%Y/%d/%m") {
                        Ok(date) => return CSVType::Date(date.to_string()),
                        Err(_) => match self.s.parse::<String>() {
                            Ok(s) => return CSVType::String(s),
                            Err(e) => return CSVType::Error(e),
                        },
                    }
                } else {
                    match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    };
                }
            },
            9 => {
                // m/dd/yyyy
                if &d[1..2] == "/" && &d[4..5] == "/" 
                || &d[1..2] == "-" && &d[4..5] == "-" {
                    match NaiveDate::parse_from_str(&*self.s.replace("-", "/"), "%m/%d/%Y") {
                        Ok(date) => return CSVType::Date(date.to_string()),
                        Err(_) => match self.s.parse::<String>() {
                            Ok(s) => return CSVType::String(s),
                            Err(e) => return CSVType::Error(e),
                        },
                    }
                }
                // mm/d/yyyy
                else if &d[2..3] == "/" && &d[4..5] == "/" 
                || &d[2..3] == "-" && &d[4..5] == "-" {
                    match NaiveDate::parse_from_str(&*self.s.replace("-", "/"), "%m/%d/%Y") {
                        Ok(date) => return CSVType::Date(date.to_string()),
                        Err(_) => match self.s.parse::<String>() {
                            Ok(s) => return CSVType::String(s),
                            Err(e) => return CSVType::Error(e),
                        },
                    }
                }
                // yyyy/mm/d
                else if &d[4..5] == "/" && &d[7..8] == "/" 
                || &d[4..5] == "-" && &d[7..8] == "-" {
                    match NaiveDate::parse_from_str(&*self.s.replace("-", "/"), "%Y/%d/%m") {
                        Ok(date) => return CSVType::Date(date.to_string()),
                        Err(_) => match self.s.parse::<String>() {
                            Ok(s) => return CSVType::String(s),
                            Err(e) => return CSVType::Error(e),
                        },
                    }
                }
                // yyyy/m/dd
                else if &d[4..5] == "/" && &d[6..7] == "/" 
                || &d[4..5] == "-" && &d[6..7] == "-" {
                    match NaiveDate::parse_from_str(&*self.s.replace("-", "/"), "%Y/%d/%m") {
                        Ok(date) => return CSVType::Date(date.to_string()),
                        Err(_) => match self.s.parse::<String>() {
                            Ok(s) => return CSVType::String(s),
                            Err(e) => return CSVType::Error(e),
                        },
                    }
                } else {
                    match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    };
                }
            },
            8 => {
                // m/d/yyyy
                if &d[1..2] == "/" && &d[3..4] == "/" 
                || &d[1..2] == "-" && &d[3..4] == "-" {
                    match NaiveDate::parse_from_str(&*self.s.replace("-", "/"), "%m/%d/%Y") {
                        Ok(date) => return CSVType::Date(date.to_string()),
                        Err(_) => match self.s.parse::<String>() {
                            Ok(s) => return CSVType::String(s),
                            Err(e) => return CSVType::Error(e),
                        },
                    }
                }
                // yyyy/m/d
                else if &d[4..5] == "/" && &d[6..7] == "/" 
                || &d[4..5] == "-" && &d[6..7] == "-" {
                    match NaiveDate::parse_from_str(&*self.s.replace("-", "/"), "%Y/%d/%m") {
                        Ok(date) => return CSVType::Date(date.to_string()),
                        Err(_) => match self.s.parse::<String>() {
                            Ok(s) => return CSVType::String(s),
                            Err(e) => return CSVType::Error(e),
                        },
                    }
                // } 
                // // mm/dd/yy
                // if &d[2..3] == "/" && &d[5..6] == "/" 
                // || &d[2..3] == "-" && &d[5..6] == "-" {
                //     return CSVType::Date((*self.s).to_string());
                } else {
                    match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    };
                }
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
        let d = &self.s.trim().to_string();
        match d.len() {
            11 => {
                // hh:mm:ss AM || PM
                if &d[2..3] == ":" && &d[5..6] == ":"
                && &d.to_uppercase()[d.len() - 1..d.len()] == "M"
                && &d.to_uppercase()[d.len() - 2..d.len() - 1] == "A"
                || &d.to_uppercase()[d.len() - 2..d.len() - 1] == "P" {
                    match NaiveTime::parse_from_str(&*self.s, "%I:%M:%S %p") {
                        Ok(time) => return CSVType::Time(time.to_string()),
                        Err(_) => match self.s.parse::<String>() {
                            Ok(s) => return CSVType::String(s),
                            Err(e) => return CSVType::Error(e),
                        },
                    }
                } else {
                    match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    };
                }
            },
            10 => {
                // h:mm:ss AM || PM
                if &d[1..2] == ":" && &d[4..5] == ":"
                && &d.to_uppercase()[d.len() - 1..d.len()] == "M"
                && &d.to_uppercase()[d.len() - 2..d.len() - 1] == "A"
                || &d.to_uppercase()[d.len() - 2..d.len() - 1] == "P" {
                    match NaiveTime::parse_from_str(&*self.s, "%I:%M:%S %p") {
                        Ok(time) => return CSVType::Time(time.to_string()),
                        Err(_) => match self.s.parse::<String>() {
                            Ok(s) => return CSVType::String(s),
                            Err(e) => return CSVType::Error(e),
                        },
                    }
                } else {
                    match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    };
                }
            },
            8 => {
                // hh:mm:ss
                if &d[2..3] == ":" && &d[5..6] == ":" {
                    match NaiveTime::parse_from_str(&*self.s, "%H:%M:%S") {
                        Ok(time) => return CSVType::Time(time.to_string()),
                        Err(_) => match self.s.parse::<String>() {
                            Ok(s) => return CSVType::String(s),
                            Err(e) => return CSVType::Error(e),
                        },
                    }
                } 
                // hh:mm AM || PM
                else if &d[2..3] == ":"
                && &d.to_uppercase()[d.len() - 1..d.len()] == "M"
                && &d.to_uppercase()[d.len() - 2..d.len() - 1] == "A"
                || &d.to_uppercase()[d.len() - 2..d.len() - 1] == "P" {
                    match NaiveTime::parse_from_str(&*self.s, "%I:%M %p") {
                        Ok(time) => return CSVType::Time(time.to_string()),
                        Err(_) => match self.s.parse::<String>() {
                            Ok(s) => return CSVType::String(s),
                            Err(e) => return CSVType::Error(e),
                        },
                    }
                } else {
                    match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    };
                }
            },
            7 => {
                // h:mm:ss
                if &d[1..2] == ":" && &d[4..5] == ":" {
                    match NaiveTime::parse_from_str(&*self.s, "%H:%M:%S") {
                        Ok(time) => return CSVType::Time(time.to_string()),
                        Err(_) => match self.s.parse::<String>() {
                            Ok(s) => return CSVType::String(s),
                            Err(e) => return CSVType::Error(e),
                        },
                    }
                }
                // h:mm AM || PM
                else if &d[1..2] == ":"
                && &d.to_uppercase()[d.len() - 1..d.len()] == "M"
                && &d.to_uppercase()[d.len() - 2..d.len() - 1] == "A"
                || &d.to_uppercase()[d.len() - 2..d.len() - 1] == "P" {
                    match NaiveTime::parse_from_str(&*self.s, "%I:%M %p") {
                        Ok(time) => return CSVType::Time(time.to_string()),
                        Err(_) => match self.s.parse::<String>() {
                            Ok(s) => return CSVType::String(s),
                            Err(e) => return CSVType::Error(e),
                        },
                    }
                } else {
                    match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    };
                }
            },
            5 => {
                // hh:mm
                if &d[2..3] == ":" {
                    match NaiveTime::parse_from_str(&*self.s, "%H:%M") {
                        Ok(time) => return CSVType::Time(time.to_string()),
                        Err(_) => match self.s.parse::<String>() {
                            Ok(s) => return CSVType::String(s),
                            Err(e) => return CSVType::Error(e),
                        },
                    }
                } else {
                    match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    };
                }
            },
            4 => {
                // h:mm
                if &d[1..2] == ":" {
                    match NaiveTime::parse_from_str(&*self.s, "%H:%M") {
                        Ok(time) => return CSVType::Time(time.to_string()),
                        Err(_) => match self.s.parse::<String>() {
                            Ok(s) => return CSVType::String(s),
                            Err(e) => return CSVType::Error(e),
                        },
                    }
                } else {
                    match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    };
                }
            },
            _ => match self.s.parse::<String>() {
                Ok(s) => return CSVType::String(s),
                Err(e) => return CSVType::Error(e),
            },
        }
    }

    pub fn datetime_match(&self) -> CSVType {
        let d = &self.s.trim().to_string();
        match d.len() {
            16 => {
                // mm/dd/yyyy hh:mm
                if &d[13..14] == ":"
                && &d[2..3] == "/" && &d[5..6] == "/" 
                || &d[2..3] == "-" && &d[5..6] == "-" {
                    match NaiveDate::parse_from_str(&*self.s.replace("-", "/"), "%m/%d/%Y %H:%M") {
                        Ok(date) => {
                            return CSVType::DateTime(date.to_string())
                        },
                        Err(_) => match self.s.parse::<String>() {
                            Ok(s) => return CSVType::String(s),
                            Err(e) => return CSVType::Error(e),
                        },
                    }
                } else {
                    match self.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(e),
                    };
                }
            },
            _ => match self.s.parse::<String>() {
                Ok(s) => return CSVType::String(s),
                Err(e) => return CSVType::Error(e),
            },
        }
    }

}