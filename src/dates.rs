use crate::csvtype::{CSVType, ByteString};
use chrono::prelude::*;

impl ByteString {
    pub fn date_match(&self) -> CSVType {
        let date = &self.s.replace("/", "-").replace(".", "-").trim().to_string();
        match date.len() {
            10 => match date {
                // mm/dd/yyyy
                d if &d[2..3] == "-" && &d[5..6] == "-"
                => match NaiveDate::parse_from_str(d, "%m-%d-%Y") {
                    Ok(date) => return CSVType::Date(date.to_string()),
                    // dd/mm/yyyy
                    Err(_) => match NaiveDate::parse_from_str(d, "%d-%m-%Y") {
                        Ok(date) => return CSVType::Date(date.to_string()),
                        Err(_) => return self.convert_to_string(),
                    },
                },
                // yyyy/mm/dd
                d if &d[4..5] == "-" && &d[7..8] == "-"
                => match NaiveDate::parse_from_str(d, "%Y-%m-%d") {
                    Ok(date) => return CSVType::Date(date.to_string()),
                    // yyyy/dd/mm
                    Err(_) => match NaiveDate::parse_from_str(d, "%Y-%d-%m") {
                        Ok(date) => return CSVType::Date(date.to_string()),
                        Err(_) => return self.convert_to_string(),
                    },
                },
                _ => return self.convert_to_string(),
            },
            9 => match date {
                // m/dd/yyyy
                d if &d[1..2] == "-" && &d[4..5] == "-" 
                => match NaiveDate::parse_from_str(d, "%m-%d-%Y") {
                    Ok(date) => return CSVType::Date(date.to_string()),
                    Err(_) => return self.convert_to_string(),
                },
                // mm/d/yyyy
                d if &d[2..3] == "-" && &d[4..5] == "-" 
                => match NaiveDate::parse_from_str(d, "%m-%d-%Y") {
                    Ok(date) => return CSVType::Date(date.to_string()),
                    // dd/m/yyyy
                    Err(_) => match NaiveDate::parse_from_str(d, "%d-%m-%Y") {
                        Ok(date) => return CSVType::Date(date.to_string()),
                        Err(_) => return self.convert_to_string(),
                    },
                },
                // yyyy/mm/d
                d if &d[4..5] == "-" && &d[7..8] == "-" 
                => match NaiveDate::parse_from_str(d, "%Y-%m-%d") {
                    Ok(date) => return CSVType::Date(date.to_string()),
                    // yyyy/dd/m
                    Err(_) => match NaiveDate::parse_from_str(d, "%Y-%d-%m") {
                        Ok(date) => return CSVType::Date(date.to_string()),
                        Err(_) => return self.convert_to_string(),
                    },
                },
                // yyyy/m/dd
                d if &d[4..5] == "-" && &d[6..7] == "-" 
                => match NaiveDate::parse_from_str(d, "%Y-%m-%d") {
                    Ok(date) => return CSVType::Date(date.to_string()),
                    Err(_) => return self.convert_to_string(),
                },
                _ => return self.convert_to_string(),
            },
            8 => match date {
                // m/d/yyyy
                d if &d[1..2] == "-" && &d[3..4] == "-" 
                => match NaiveDate::parse_from_str(d, "%m-%d-%Y") {
                    Ok(date) => return CSVType::Date(date.to_string()),
                    Err(_) => return self.convert_to_string(),
                },
                // yyyy/m/d
                d if &d[4..5] == "-" && &d[6..7] == "-" 
                => match NaiveDate::parse_from_str(d, "%Y-%m-%d") {
                    Ok(date) => return CSVType::Date(date.to_string()),
                    Err(_) => return self.convert_to_string(),
                },
                // mm/dd/yy
                d if &d[2..3] == "-" && &d[5..6] == "-"
                => match NaiveDate::parse_from_str(d, "%m-%d-%y") {
                    Ok(date) => return CSVType::Date(date.to_string()),
                    // dd/mm/yy
                    Err(_) => match NaiveDate::parse_from_str(d, "%d-%m-%y") {
                        Ok(date) => return CSVType::Date(date.to_string()),
                        Err(_) => return self.convert_to_string(),
                    },
                },
                _ => return self.convert_to_string(),
            },
            7 => match date {
                // m/dd/yy
                d if &d[1..2] == "-" && &d[4..5] == "-"
                => match NaiveDate::parse_from_str(d, "%m-%d-%y") {
                    Ok(date) => return CSVType::Date(date.to_string()),
                    Err(_) => return self.convert_to_string(),
                },
                // mm/d/yy
                d if &d[2..3] == "-" && &d[4..5] == "-"
                => match NaiveDate::parse_from_str(d, "%m-%d-%y") {
                    Ok(date) => return CSVType::Date(date.to_string()),
                    // dd/m/yy
                    Err(_) => match NaiveDate::parse_from_str(d, "%d-%m-%y") {
                        Ok(date) => return CSVType::Date(date.to_string()),
                        // yy/m/dd
                        Err(_) => match NaiveDate::parse_from_str(d, "%y-%m-%d") {
                            Ok(date) => return CSVType::Date(date.to_string()),
                            Err(_) => return self.convert_to_string(),
                        },
                    },
                },
                // yy/mm/d
                d if &d[2..3] == "-" && &d[5..6] == "-"
                => match NaiveDate::parse_from_str(d, "%y-%m-%d") {
                    Ok(date) => return CSVType::Date(date.to_string()),
                    // yy/dd/m
                    Err(_) => match NaiveDate::parse_from_str(d, "%y-%d-%m") {
                        Ok(date) => return CSVType::Date(date.to_string()),
                        Err(_) => return self.convert_to_string(),
                    },
                },
                _ => return self.convert_to_string(),
            },
            6 => match date {
                // m/d/yy
                d if &d[1..2] == "-" && &d[3..4] == "-"
                => match NaiveDate::parse_from_str(d, "%m-%d-%y") {
                    Ok(date) => return CSVType::Date(date.to_string()),
                    Err(_) => return self.convert_to_string(),
                },
                // yy/m/d
                d if &d[2..3] == "-" && &d[4..5] == "-"
                => match NaiveDate::parse_from_str(d, "%y-%m-%d") {
                    Ok(date) => return CSVType::Date(date.to_string()),
                    Err(_) => return self.convert_to_string(),
                },
                _ => return self.convert_to_string(),
            },
            _ => return self.convert_to_string(),
        }
    }   

    pub fn time_match(&self) -> CSVType {
        let time = &self.s.trim().to_string();
        match time {
            t if t.to_ascii_uppercase().contains("AM") || t.to_ascii_uppercase().contains("PM") 
            => match t {
                // hh:mm AM && h:mm AM
                t if t.len() < 9
                => match NaiveTime::parse_from_str(&t.to_ascii_uppercase(), "%I:%M %p") {
                    Ok(time) => return CSVType::Time(time.to_string()),
                    Err(_) => return self.convert_to_string(),
                },
                // hh:mm:ss AM && h:mm:ss AM
                _ => match NaiveTime::parse_from_str(&t.to_ascii_uppercase(), "%I:%M:%S %p") {
                    Ok(time) => return CSVType::Time(time.to_string()),
                    Err(_) => return self.convert_to_string(),
                },
            },
            t =>  match t {
                // hh:mm && h:mm
                t if t.len() < 6
                => match NaiveTime::parse_from_str(&*self.s, "%H:%M") {
                    Ok(time) => return CSVType::Time(time.to_string()),
                    Err(_) => return self.convert_to_string(),
                },
                // hh:mm:ss && h:mm:ss
                _ => match NaiveTime::parse_from_str(&*self.s, "%H:%M:%S") {
                    Ok(time) => return CSVType::Time(time.to_string()),
                    Err(_) => return self.convert_to_string(),
                },
            },
        }
    }

    pub fn datetime_match(&self) -> CSVType {
        let datetime = &self.s.replace("/", "-").replace(".", "-").trim().to_string();
        match datetime {
            dt if dt.to_ascii_uppercase().contains("AM") || dt.to_ascii_uppercase().contains("PM")
            => match datetime.len() {
                19 => match datetime {
                    // mm/dd/yyyy hh:mm AM
                    dt if &dt[13..14] == ":"
                    && &dt[2..3] == "-" && &dt[5..6] == "-" 
                    => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%m-%d-%Y %I:%M %p") {
                        Ok(date) => return CSVType::DateTime(date.to_string()),
                        // dd/mm/yyyy hh:mm AM
                        Err(_) => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%d-%m-%Y %I:%M %p") {
                            Ok(date) => return CSVType::DateTime(date.to_string()),
                            Err(_) => return self.convert_to_string(),
                        },
                    },
                    // yyyy/mm/dd hh:mm AM
                    dt if &dt[13..14] == ":"
                    && &dt[4..5] == "-" && &dt[7..8] == "-" 
                    => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%Y-%m-%d %I:%M %p") {
                        Ok(date) => return CSVType::DateTime(date.to_string()),
                        // yyyy/dd/mm hh:mm AM
                        Err(_) => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%Y-%d-%m %I:%M %p") {
                            Ok(date) => return CSVType::DateTime(date.to_string()),
                            Err(_) => return self.convert_to_string(),
                        },
                    },
                    _ => return self.convert_to_string(),
                },
                18 => match datetime {
                    // mm/dd/yyyy h:mm AM
                    dt if &dt[12..13] == ":"
                    && &dt[2..3] == "-" && &dt[5..6] == "-" 
                    => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%m-%d-%Y %I:%M %p") {
                        Ok(date) => return CSVType::DateTime(date.to_string()),
                        // dd/mm/yyyy h:mm AM
                        Err(_) => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%d-%m-%Y %I:%M %p") {
                            Ok(date) => return CSVType::DateTime(date.to_string()),
                            Err(_) => return self.convert_to_string(),
                        },
                    },
                    // m/dd/yyyy hh:mm AM
                    dt if &dt[12..13] == ":"
                    && &dt[1..2] == "-" && &dt[4..5] == "-" 
                    => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%m-%d-%Y %I:%M %p") {
                        Ok(date) => return CSVType::DateTime(date.to_string()),
                        Err(_) => return self.convert_to_string(),
                    },
                    // mm/d/yyyy hh:mm AM
                    dt if &dt[12..13] == ":"
                    && &dt[2..3] == "-" && &dt[4..5] == "-" 
                    => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%m-%d-%Y %I:%M %p") {
                        Ok(date) => return CSVType::DateTime(date.to_string()),
                        // dd/m/yyyy hh:mm AM
                        Err(_) => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%d-%m-%Y %I:%M %p") {
                            Ok(date) => return CSVType::DateTime(date.to_string()),
                            Err(_) => return self.convert_to_string(),
                        },
                    },
                    _ => return self.convert_to_string(),
                },
                17 => match datetime {
                    // m/dd/yyyy h:mm AM
                    dt if &dt[11..12] == ":"
                    && &dt[1..2] == "-" && &dt[4..5] == "-" 
                    => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%m-%d-%Y %I:%M %p") {
                        Ok(date) => return CSVType::DateTime(date.to_string()),
                        Err(_) => return self.convert_to_string(),
                    },
                    // mm/d/yyyy h:mm AM
                    dt if &dt[11..12] == ":"
                    && &dt[2..3] == "-" && &dt[4..5] == "-" 
                    => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%m-%d-%Y %I:%M %p") {
                        Ok(date) => return CSVType::DateTime(date.to_string()),
                        // dd/m/yyyy h:mm AM
                        Err(_) => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%d-%m-%Y %I:%M %p") {
                            Ok(date) => return CSVType::DateTime(date.to_string()),
                            Err(_) => return self.convert_to_string(),
                        },
                    },
                    // m/d/yyyy hh:mm AM
                    dt if &dt[11..12] == ":"
                    && &dt[1..2] == "-" && &dt[3..4] == "-" 
                    => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%m-%d-%Y %I:%M %p") {
                        Ok(date) => return CSVType::DateTime(date.to_string()),
                        Err(_) => return self.convert_to_string(),
                    },
                    // mm/dd/yy hh:mm AM
                    dt if &dt[11..12] == ":"
                    && &dt[2..3] == "-" && &dt[5..6] == "-" 
                    => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%m-%d-%y %I:%M %p") {
                        Ok(date) => return CSVType::DateTime(date.to_string()),
                        // dd/mm/yy hh:mm AM
                        Err(_) => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%d-%m-%y %I:%M %p") {
                            Ok(date) => return CSVType::DateTime(date.to_string()),
                            // yy/mm/dd hh:mm AM
                            Err(_) => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%y-%m-%d %I:%M %p") {
                                Ok(date) => return CSVType::DateTime(date.to_string()),
                                // yy/dd/mm hh:mm AM
                                Err(_) => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%y-%d-%m %I:%M %p") {
                                    Ok(date) => return CSVType::DateTime(date.to_string()),
                                    Err(_) => return self.convert_to_string(),
                                },
                            },
                        },
                    },
                    _ => return self.convert_to_string(),
                },
                16 => match datetime {
                    // m/d/yyyy h:mm AM
                    dt if &dt[10..11] == ":"
                    && &dt[1..2] == "-" && &dt[3..4] == "-" 
                    => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%m-%d-%Y %I:%M %p") {
                        Ok(date) => return CSVType::DateTime(date.to_string()),
                        Err(_) => return self.convert_to_string(),
                    },
                    // yyyy/m/d h:mm AM
                    dt if &dt[10..11] == ":"
                    && &dt[4..5] == "-" && &dt[6..7] == "-"  
                    => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%Y-%m-%d %I:%M %p") {
                        Ok(date) => return CSVType::DateTime(date.to_string()),
                        Err(_) => return self.convert_to_string(),
                    },
                    // mm/dd/yy h:mm AM
                    dt if &dt[10..11] == ":"
                    && &dt[2..3] == "-" && &dt[5..6] == "-" 
                    => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%m-%d-%y %I:%M %p") {
                        Ok(date) => return CSVType::DateTime(date.to_string()),
                        // dd/mm/yy h:mm AM
                        Err(_) => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%d-%m-%y %I:%M %p") {
                            Ok(date) => return CSVType::DateTime(date.to_string()),
                            // yy/mm/dd h:mm AM
                            Err(_) => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%y-%m-%d %I:%M %p") {
                                Ok(date) => return CSVType::DateTime(date.to_string()),
                                // yy/dd/mm h:mm AM
                                Err(_) => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%y-%d-%m %I:%M %p") {
                                    Ok(date) => return CSVType::DateTime(date.to_string()),
                                    Err(_) => return self.convert_to_string(),
                                },
                            },
                        },
                    },
                    // m/dd/yy hh:mm AM
                    dt if &dt[10..11] == ":"
                    && &dt[1..2] == "-" && &dt[4..5] == "-" 
                    => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%m-%d-%y %I:%M %p") {
                        Ok(date) => return CSVType::DateTime(date.to_string()),
                        Err(_) => return self.convert_to_string(),
                    },
                    // mm/d/yy hh:mm AM
                    dt if &dt[10..11] == ":"
                    && &dt[2..3] == "-" && &dt[4..5] == "-" 
                    => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%m-%d-%y %I:%M %p") {
                        Ok(date) => return CSVType::DateTime(date.to_string()),
                        // dd/m/yy hh:mm AM
                        Err(_) => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%d-%m-%y %I:%M %p") {
                            Ok(date) => return CSVType::DateTime(date.to_string()),
                            // yy/m/dd hh:mm AM
                            Err(_) => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%y-%m-%d %I:%M %p") {
                                Ok(date) => return CSVType::DateTime(date.to_string()),
                                // yy/d/mm hh:mm AM
                                Err(_) => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%y-%d-%m %I:%M %p") {
                                    Ok(date) => return CSVType::DateTime(date.to_string()),
                                    Err(_) => return self.convert_to_string(),
                                },
                            },
                        },
                    },
                    _ => return self.convert_to_string(),
                },
                15 => match datetime {
                    // m/dd/yy h:mm AM
                    dt if &dt[9..10] == ":"
                    && &dt[1..2] == "-" && &dt[4..5] == "-" 
                    => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%m-%d-%y %I:%M %p") {
                        Ok(date) => return CSVType::DateTime(date.to_string()),
                        Err(_) => return self.convert_to_string(),
                    },
                    // mm/d/yy h:mm AM
                    dt if &dt[9..10] == ":"
                    && &dt[2..3] == "-" && &dt[4..5] == "-" 
                    => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%m-%d-%y %I:%M %p") {
                        Ok(date) => return CSVType::DateTime(date.to_string()),
                        // dd/m/yy h:mm AM
                        Err(_) => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%d-%m-%y %I:%M %p") {
                            Ok(date) => return CSVType::DateTime(date.to_string()),
                            // yy/m/dd h:mm AM
                            Err(_) => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%y-%m-%d %I:%M %p") {
                                Ok(date) => return CSVType::DateTime(date.to_string()),
                                // yy/d/mm h:mm AM
                                Err(_) => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%y-%d-%m %I:%M %p") {
                                    Ok(date) => return CSVType::DateTime(date.to_string()),
                                    Err(_) => return self.convert_to_string(),
                                },
                            },
                        },
                    },
                    _ => return self.convert_to_string(),
                },
                _ => return self.convert_to_string(),
            },
            _ => match datetime.len() {
                16 => match datetime {
                    // mm/dd/yyyy hh:mm
                    dt if &dt[13..14] == ":"
                    && &dt[2..3] == "-" && &dt[5..6] == "-" 
                    => match NaiveDateTime::parse_from_str(dt, "%m-%d-%Y %H:%M") {
                        Ok(date) => return CSVType::DateTime(date.to_string()),
                        // dd/mm/yyyy hh:mm
                        Err(_) => match NaiveDateTime::parse_from_str(dt, "%d-%m-%Y %H:%M") {
                            Ok(date) => return CSVType::DateTime(date.to_string()),
                            Err(_) => return self.convert_to_string(),
                        },
                    },
                    // yyyy/mm/dd hh:mm
                    dt if &dt[13..14] == ":"
                    && &dt[4..5] == "-" && &dt[7..8] == "-" 
                    => match NaiveDateTime::parse_from_str(dt, "%Y-%m-%d %H:%M") {
                        Ok(date) => return CSVType::DateTime(date.to_string()),
                        // yyyy/dd/mm hh:mm
                        Err(_) => match NaiveDateTime::parse_from_str(dt, "%Y-%d-%m %H:%M") {
                            Ok(date) => return CSVType::DateTime(date.to_string()),
                            Err(_) => return self.convert_to_string(),
                        },
                    },
                    _ => return self.convert_to_string(),
                },
                15 => match datetime {
                    // mm/dd/yyyy h:mm
                    dt if &dt[12..13] == ":"
                    && &dt[2..3] == "-" && &dt[5..6] == "-" 
                    => match NaiveDateTime::parse_from_str(dt, "%m-%d-%Y %H:%M") {
                        Ok(date) => return CSVType::DateTime(date.to_string()),
                        // dd/mm/yyyy h:mm
                        Err(_) => match NaiveDateTime::parse_from_str(dt, "%d-%m-%Y %H:%M") {
                            Ok(date) => return CSVType::DateTime(date.to_string()),
                            Err(_) => return self.convert_to_string(),
                        },
                    },
                    // m/dd/yyyy hh:mm
                    dt if &dt[12..13] == ":"
                    && &dt[1..2] == "-" && &dt[4..5] == "-" 
                    => match NaiveDateTime::parse_from_str(dt, "%m-%d-%Y %H:%M") {
                        Ok(date) => return CSVType::DateTime(date.to_string()),
                        Err(_) => return self.convert_to_string(),
                    },
                    // mm/d/yyyy hh:mm
                    dt if &dt[12..13] == ":"
                    && &dt[2..3] == "-" && &dt[4..5] == "-" 
                    => match NaiveDateTime::parse_from_str(dt, "%m-%d-%Y %H:%M") {
                        Ok(date) => return CSVType::DateTime(date.to_string()),
                        // dd/m/yyyy hh:mm
                        Err(_) => match NaiveDateTime::parse_from_str(dt, "%d-%m-%Y %H:%M") {
                            Ok(date) => return CSVType::DateTime(date.to_string()),
                            Err(_) => return self.convert_to_string(),
                        },
                    },
                    _ => return self.convert_to_string(),
                },
                14 => match datetime {
                    // m/dd/yyyy h:mm
                    dt if &dt[11..12] == ":"
                    && &dt[1..2] == "-" && &dt[4..5] == "-" 
                    => match NaiveDateTime::parse_from_str(dt, "%m-%d-%Y %H:%M") {
                        Ok(date) => return CSVType::DateTime(date.to_string()),
                        Err(_) => return self.convert_to_string(),
                    },
                    // mm/d/yyyy h:mm
                    dt if &dt[11..12] == ":"
                    && &dt[2..3] == "-" && &dt[4..5] == "-" 
                    => match NaiveDateTime::parse_from_str(dt, "%m-%d-%Y %H:%M") {
                        Ok(date) => return CSVType::DateTime(date.to_string()),
                        // dd/m/yyyy h:mm
                        Err(_) => match NaiveDateTime::parse_from_str(dt, "%d-%m-%Y %H:%M") {
                            Ok(date) => return CSVType::DateTime(date.to_string()),
                            Err(_) => return self.convert_to_string(),
                        },
                    },
                    // m/d/yyyy hh:mm
                    dt if &dt[11..12] == ":"
                    && &dt[1..2] == "-" && &dt[3..4] == "-" 
                    => match NaiveDateTime::parse_from_str(dt, "%m-%d-%Y %H:%M") {
                        Ok(date) => return CSVType::DateTime(date.to_string()),
                        Err(_) => return self.convert_to_string(),
                    },
                    // mm/dd/yy hh:mm
                    dt if &dt[11..12] == ":"
                    && &dt[2..3] == "-" && &dt[5..6] == "-" 
                    => match NaiveDateTime::parse_from_str(dt, "%m-%d-%y %H:%M") {
                        Ok(date) => return CSVType::DateTime(date.to_string()),
                        // dd/mm/yy hh:mm
                        Err(_) => match NaiveDateTime::parse_from_str(dt, "%d-%m-%y %H:%M") {
                            Ok(date) => return CSVType::DateTime(date.to_string()),
                            // yy/mm/dd hh:mm
                            Err(_) => match NaiveDateTime::parse_from_str(dt, "%y-%m-%d %H:%M") {
                                Ok(date) => return CSVType::DateTime(date.to_string()),
                                // yy/dd/mm hh:mm
                                Err(_) => match NaiveDateTime::parse_from_str(dt, "%y-%d-%m %H:%M") {
                                    Ok(date) => return CSVType::DateTime(date.to_string()),
                                    Err(_) => return self.convert_to_string(),
                                },
                            },
                        },
                    },
                    _ => return self.convert_to_string(),
                },
                13 => match datetime {
                    // m/d/yyyy h:mm
                    dt if &dt[10..11] == ":"
                    && &dt[1..2] == "-" && &dt[3..4] == "-" 
                    => match NaiveDateTime::parse_from_str(dt, "%m-%d-%Y %H:%M") {
                        Ok(date) => return CSVType::DateTime(date.to_string()),
                        Err(_) => return self.convert_to_string(),
                    },
                    // yyyy/m/d h:mm
                    dt if &dt[10..11] == ":"
                    && &dt[4..5] == "-" && &dt[6..7] == "-"  
                    => match NaiveDateTime::parse_from_str(dt, "%Y-%m-%d %H:%M") {
                        Ok(date) => return CSVType::DateTime(date.to_string()),
                        Err(_) => return self.convert_to_string(),
                    },
                    // mm/dd/yy h:mm
                    dt if &dt[10..11] == ":"
                    && &dt[2..3] == "-" && &dt[5..6] == "-" 
                    => match NaiveDateTime::parse_from_str(dt, "%m-%d-%y %H:%M") {
                        Ok(date) => return CSVType::DateTime(date.to_string()),
                        // dd/mm/yy h:mm
                        Err(_) => match NaiveDateTime::parse_from_str(dt, "%d-%m-%y %H:%M") {
                            Ok(date) => return CSVType::DateTime(date.to_string()),
                            // yy/mm/dd h:mm
                            Err(_) => match NaiveDateTime::parse_from_str(dt, "%y-%m-%d %H:%M") {
                                Ok(date) => return CSVType::DateTime(date.to_string()),
                                // yy/dd/mm h:mm
                                Err(_) => match NaiveDateTime::parse_from_str(dt, "%y-%d-%m %H:%M") {
                                    Ok(date) => return CSVType::DateTime(date.to_string()),
                                    Err(_) => return self.convert_to_string(),
                                },
                            },
                        },
                    },
                    // m/dd/yy hh:mm
                    dt if &dt[10..11] == ":"
                    && &dt[1..2] == "-" && &dt[4..5] == "-" 
                    => match NaiveDateTime::parse_from_str(dt, "%m-%d-%y %H:%M") {
                        Ok(date) => return CSVType::DateTime(date.to_string()),
                        Err(_) => return self.convert_to_string(),
                    },
                    // mm/d/yy hh:mm
                    dt if &dt[10..11] == ":"
                    && &dt[2..3] == "-" && &dt[4..5] == "-" 
                    => match NaiveDateTime::parse_from_str(dt, "%m-%d-%y %H:%M") {
                        Ok(date) => return CSVType::DateTime(date.to_string()),
                        // dd/m/yy hh:mm
                        Err(_) => match NaiveDateTime::parse_from_str(dt, "%d-%m-%y %H:%M") {
                            Ok(date) => return CSVType::DateTime(date.to_string()),
                            // yy/m/dd hh:mm
                            Err(_) => match NaiveDateTime::parse_from_str(dt, "%y-%m-%d %H:%M") {
                                Ok(date) => return CSVType::DateTime(date.to_string()),
                                // yy/d/mm hh:mm
                                Err(_) => match NaiveDateTime::parse_from_str(dt, "%y-%d-%m %H:%M") {
                                    Ok(date) => return CSVType::DateTime(date.to_string()),
                                    Err(_) => return self.convert_to_string(),
                                },
                            },
                        },
                    },
                    _ => return self.convert_to_string(),
                },
                12 => match datetime {
                    // m/dd/yy h:mm
                    dt if &dt[9..10] == ":"
                    && &dt[1..2] == "-" && &dt[4..5] == "-" 
                    => match NaiveDateTime::parse_from_str(dt, "%m-%d-%y %H:%M") {
                        Ok(date) => return CSVType::DateTime(date.to_string()),
                        Err(_) => return self.convert_to_string(),
                    },
                    // mm/d/yy h:mm
                    dt if &dt[9..10] == ":"
                    && &dt[2..3] == "-" && &dt[4..5] == "-" 
                    => match NaiveDateTime::parse_from_str(dt, "%m-%d-%y %H:%M") {
                        Ok(date) => return CSVType::DateTime(date.to_string()),
                        // dd/m/yy h:mm
                        Err(_) => match NaiveDateTime::parse_from_str(dt, "%d-%m-%y %H:%M") {
                            Ok(date) => return CSVType::DateTime(date.to_string()),
                            // yy/m/dd h:mm
                            Err(_) => match NaiveDateTime::parse_from_str(dt, "%y-%m-%d %H:%M") {
                                Ok(date) => return CSVType::DateTime(date.to_string()),
                                // yy/d/mm h:mm
                                Err(_) => match NaiveDateTime::parse_from_str(dt, "%y-%d-%m %H:%M") {
                                    Ok(date) => return CSVType::DateTime(date.to_string()),
                                    Err(_) => return self.convert_to_string(),
                                },
                            },
                        },
                    },
                    _ => return self.convert_to_string(),
                },
                _ => return self.convert_to_string(),
            },
        }
    }

    pub fn date_w_abbrv_match(&self) -> CSVType {
        let date = &self.s.replace("/", "-").replace(".", "-").trim().to_string();
        match date.len() {
            11 => match date {
                // dd/mmm/yyyy
                dt if &dt[2..3] == "-" && &dt[6..7] == "-" 
                => match NaiveDate::parse_from_str(dt, "%d-%b-%Y") {
                    Ok(date) => return CSVType::Date(date.to_string()),
                    Err(_) => return self.convert_to_string(),
                },
                _ => return self.convert_to_string(),
            },
            9 => match date {
                // dd/mmm/yy
                dt if &dt[2..3] == "-" && &dt[6..7] == "-" 
                => match NaiveDate::parse_from_str(dt, "%d-%b-%y") {
                    Ok(date) => return CSVType::Date(date.to_string()),
                    Err(_) => return self.convert_to_string(),
                },
                _ => return self.convert_to_string(),
            },
            _ => return self.convert_to_string(),
        }
    }
}