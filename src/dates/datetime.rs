use chrono::NaiveDateTime;

use crate::{types::ByteString, CSVType};

impl ByteString {
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
                        Ok(date) => return CSVType::DateTime(date),
                        // dd/mm/yyyy hh:mm AM
                        Err(_) => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%d-%m-%Y %I:%M %p") {
                            Ok(date) => return CSVType::DateTime(date),
                            Err(_) => return self.convert_to_string(),
                        },
                    },
                    // yyyy/mm/dd hh:mm AM
                    dt if &dt[13..14] == ":"
                    && &dt[4..5] == "-" && &dt[7..8] == "-" 
                    => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%Y-%m-%d %I:%M %p") {
                        Ok(date) => return CSVType::DateTime(date),
                        // yyyy/dd/mm hh:mm AM
                        Err(_) => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%Y-%d-%m %I:%M %p") {
                            Ok(date) => return CSVType::DateTime(date),
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
                        Ok(date) => return CSVType::DateTime(date),
                        // dd/mm/yyyy h:mm AM
                        Err(_) => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%d-%m-%Y %I:%M %p") {
                            Ok(date) => return CSVType::DateTime(date),
                            Err(_) => return self.convert_to_string(),
                        },
                    },
                    // m/dd/yyyy hh:mm AM
                    dt if &dt[12..13] == ":"
                    && &dt[1..2] == "-" && &dt[4..5] == "-" 
                    => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%m-%d-%Y %I:%M %p") {
                        Ok(date) => return CSVType::DateTime(date),
                        Err(_) => return self.convert_to_string(),
                    },
                    // mm/d/yyyy hh:mm AM
                    dt if &dt[12..13] == ":"
                    && &dt[2..3] == "-" && &dt[4..5] == "-" 
                    => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%m-%d-%Y %I:%M %p") {
                        Ok(date) => return CSVType::DateTime(date),
                        // dd/m/yyyy hh:mm AM
                        Err(_) => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%d-%m-%Y %I:%M %p") {
                            Ok(date) => return CSVType::DateTime(date),
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
                        Ok(date) => return CSVType::DateTime(date),
                        Err(_) => return self.convert_to_string(),
                    },
                    // mm/d/yyyy h:mm AM
                    dt if &dt[11..12] == ":"
                    && &dt[2..3] == "-" && &dt[4..5] == "-" 
                    => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%m-%d-%Y %I:%M %p") {
                        Ok(date) => return CSVType::DateTime(date),
                        // dd/m/yyyy h:mm AM
                        Err(_) => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%d-%m-%Y %I:%M %p") {
                            Ok(date) => return CSVType::DateTime(date),
                            Err(_) => return self.convert_to_string(),
                        },
                    },
                    // m/d/yyyy hh:mm AM
                    dt if &dt[11..12] == ":"
                    && &dt[1..2] == "-" && &dt[3..4] == "-" 
                    => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%m-%d-%Y %I:%M %p") {
                        Ok(date) => return CSVType::DateTime(date),
                        Err(_) => return self.convert_to_string(),
                    },
                    // mm/dd/yy hh:mm AM
                    dt if &dt[11..12] == ":"
                    && &dt[2..3] == "-" && &dt[5..6] == "-" 
                    => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%m-%d-%y %I:%M %p") {
                        Ok(date) => return CSVType::DateTime(date),
                        // dd/mm/yy hh:mm AM
                        Err(_) => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%d-%m-%y %I:%M %p") {
                            Ok(date) => return CSVType::DateTime(date),
                            // yy/mm/dd hh:mm AM
                            Err(_) => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%y-%m-%d %I:%M %p") {
                                Ok(date) => return CSVType::DateTime(date),
                                // yy/dd/mm hh:mm AM
                                Err(_) => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%y-%d-%m %I:%M %p") {
                                    Ok(date) => return CSVType::DateTime(date),
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
                        Ok(date) => return CSVType::DateTime(date),
                        Err(_) => return self.convert_to_string(),
                    },
                    // yyyy/m/d h:mm AM
                    dt if &dt[10..11] == ":"
                    && &dt[4..5] == "-" && &dt[6..7] == "-"  
                    => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%Y-%m-%d %I:%M %p") {
                        Ok(date) => return CSVType::DateTime(date),
                        Err(_) => return self.convert_to_string(),
                    },
                    // mm/dd/yy h:mm AM
                    dt if &dt[10..11] == ":"
                    && &dt[2..3] == "-" && &dt[5..6] == "-" 
                    => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%m-%d-%y %I:%M %p") {
                        Ok(date) => return CSVType::DateTime(date),
                        // dd/mm/yy h:mm AM
                        Err(_) => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%d-%m-%y %I:%M %p") {
                            Ok(date) => return CSVType::DateTime(date),
                            // yy/mm/dd h:mm AM
                            Err(_) => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%y-%m-%d %I:%M %p") {
                                Ok(date) => return CSVType::DateTime(date),
                                // yy/dd/mm h:mm AM
                                Err(_) => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%y-%d-%m %I:%M %p") {
                                    Ok(date) => return CSVType::DateTime(date),
                                    Err(_) => return self.convert_to_string(),
                                },
                            },
                        },
                    },
                    // m/dd/yy hh:mm AM
                    dt if &dt[10..11] == ":"
                    && &dt[1..2] == "-" && &dt[4..5] == "-" 
                    => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%m-%d-%y %I:%M %p") {
                        Ok(date) => return CSVType::DateTime(date),
                        Err(_) => return self.convert_to_string(),
                    },
                    // mm/d/yy hh:mm AM
                    dt if &dt[10..11] == ":"
                    && &dt[2..3] == "-" && &dt[4..5] == "-" 
                    => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%m-%d-%y %I:%M %p") {
                        Ok(date) => return CSVType::DateTime(date),
                        // dd/m/yy hh:mm AM
                        Err(_) => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%d-%m-%y %I:%M %p") {
                            Ok(date) => return CSVType::DateTime(date),
                            // yy/m/dd hh:mm AM
                            Err(_) => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%y-%m-%d %I:%M %p") {
                                Ok(date) => return CSVType::DateTime(date),
                                // yy/d/mm hh:mm AM
                                Err(_) => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%y-%d-%m %I:%M %p") {
                                    Ok(date) => return CSVType::DateTime(date),
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
                        Ok(date) => return CSVType::DateTime(date),
                        Err(_) => return self.convert_to_string(),
                    },
                    // mm/d/yy h:mm AM
                    dt if &dt[9..10] == ":"
                    && &dt[2..3] == "-" && &dt[4..5] == "-" 
                    => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%m-%d-%y %I:%M %p") {
                        Ok(date) => return CSVType::DateTime(date),
                        // dd/m/yy h:mm AM
                        Err(_) => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%d-%m-%y %I:%M %p") {
                            Ok(date) => return CSVType::DateTime(date),
                            // yy/m/dd h:mm AM
                            Err(_) => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%y-%m-%d %I:%M %p") {
                                Ok(date) => return CSVType::DateTime(date),
                                // yy/d/mm h:mm AM
                                Err(_) => match NaiveDateTime::parse_from_str(&dt.to_ascii_uppercase(), "%y-%d-%m %I:%M %p") {
                                    Ok(date) => return CSVType::DateTime(date),
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
                        Ok(date) => return CSVType::DateTime(date),
                        // dd/mm/yyyy hh:mm
                        Err(_) => match NaiveDateTime::parse_from_str(dt, "%d-%m-%Y %H:%M") {
                            Ok(date) => return CSVType::DateTime(date),
                            Err(_) => return self.convert_to_string(),
                        },
                    },
                    // yyyy/mm/dd hh:mm
                    dt if &dt[13..14] == ":"
                    && &dt[4..5] == "-" && &dt[7..8] == "-" 
                    => match NaiveDateTime::parse_from_str(dt, "%Y-%m-%d %H:%M") {
                        Ok(date) => return CSVType::DateTime(date),
                        // yyyy/dd/mm hh:mm
                        Err(_) => match NaiveDateTime::parse_from_str(dt, "%Y-%d-%m %H:%M") {
                            Ok(date) => return CSVType::DateTime(date),
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
                        Ok(date) => return CSVType::DateTime(date),
                        // dd/mm/yyyy h:mm
                        Err(_) => match NaiveDateTime::parse_from_str(dt, "%d-%m-%Y %H:%M") {
                            Ok(date) => return CSVType::DateTime(date),
                            Err(_) => return self.convert_to_string(),
                        },
                    },
                    // m/dd/yyyy hh:mm
                    dt if &dt[12..13] == ":"
                    && &dt[1..2] == "-" && &dt[4..5] == "-" 
                    => match NaiveDateTime::parse_from_str(dt, "%m-%d-%Y %H:%M") {
                        Ok(date) => return CSVType::DateTime(date),
                        Err(_) => return self.convert_to_string(),
                    },
                    // mm/d/yyyy hh:mm
                    dt if &dt[12..13] == ":"
                    && &dt[2..3] == "-" && &dt[4..5] == "-" 
                    => match NaiveDateTime::parse_from_str(dt, "%m-%d-%Y %H:%M") {
                        Ok(date) => return CSVType::DateTime(date),
                        // dd/m/yyyy hh:mm
                        Err(_) => match NaiveDateTime::parse_from_str(dt, "%d-%m-%Y %H:%M") {
                            Ok(date) => return CSVType::DateTime(date),
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
                        Ok(date) => return CSVType::DateTime(date),
                        Err(_) => return self.convert_to_string(),
                    },
                    // mm/d/yyyy h:mm
                    dt if &dt[11..12] == ":"
                    && &dt[2..3] == "-" && &dt[4..5] == "-" 
                    => match NaiveDateTime::parse_from_str(dt, "%m-%d-%Y %H:%M") {
                        Ok(date) => return CSVType::DateTime(date),
                        // dd/m/yyyy h:mm
                        Err(_) => match NaiveDateTime::parse_from_str(dt, "%d-%m-%Y %H:%M") {
                            Ok(date) => return CSVType::DateTime(date),
                            Err(_) => return self.convert_to_string(),
                        },
                    },
                    // m/d/yyyy hh:mm
                    dt if &dt[11..12] == ":"
                    && &dt[1..2] == "-" && &dt[3..4] == "-" 
                    => match NaiveDateTime::parse_from_str(dt, "%m-%d-%Y %H:%M") {
                        Ok(date) => return CSVType::DateTime(date),
                        Err(_) => return self.convert_to_string(),
                    },
                    // mm/dd/yy hh:mm
                    dt if &dt[11..12] == ":"
                    && &dt[2..3] == "-" && &dt[5..6] == "-" 
                    => match NaiveDateTime::parse_from_str(dt, "%m-%d-%y %H:%M") {
                        Ok(date) => return CSVType::DateTime(date),
                        // dd/mm/yy hh:mm
                        Err(_) => match NaiveDateTime::parse_from_str(dt, "%d-%m-%y %H:%M") {
                            Ok(date) => return CSVType::DateTime(date),
                            // yy/mm/dd hh:mm
                            Err(_) => match NaiveDateTime::parse_from_str(dt, "%y-%m-%d %H:%M") {
                                Ok(date) => return CSVType::DateTime(date),
                                // yy/dd/mm hh:mm
                                Err(_) => match NaiveDateTime::parse_from_str(dt, "%y-%d-%m %H:%M") {
                                    Ok(date) => return CSVType::DateTime(date),
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
                        Ok(date) => return CSVType::DateTime(date),
                        Err(_) => return self.convert_to_string(),
                    },
                    // yyyy/m/d h:mm
                    dt if &dt[10..11] == ":"
                    && &dt[4..5] == "-" && &dt[6..7] == "-"  
                    => match NaiveDateTime::parse_from_str(dt, "%Y-%m-%d %H:%M") {
                        Ok(date) => return CSVType::DateTime(date),
                        Err(_) => return self.convert_to_string(),
                    },
                    // mm/dd/yy h:mm
                    dt if &dt[10..11] == ":"
                    && &dt[2..3] == "-" && &dt[5..6] == "-" 
                    => match NaiveDateTime::parse_from_str(dt, "%m-%d-%y %H:%M") {
                        Ok(date) => return CSVType::DateTime(date),
                        // dd/mm/yy h:mm
                        Err(_) => match NaiveDateTime::parse_from_str(dt, "%d-%m-%y %H:%M") {
                            Ok(date) => return CSVType::DateTime(date),
                            // yy/mm/dd h:mm
                            Err(_) => match NaiveDateTime::parse_from_str(dt, "%y-%m-%d %H:%M") {
                                Ok(date) => return CSVType::DateTime(date),
                                // yy/dd/mm h:mm
                                Err(_) => match NaiveDateTime::parse_from_str(dt, "%y-%d-%m %H:%M") {
                                    Ok(date) => return CSVType::DateTime(date),
                                    Err(_) => return self.convert_to_string(),
                                },
                            },
                        },
                    },
                    // m/dd/yy hh:mm
                    dt if &dt[10..11] == ":"
                    && &dt[1..2] == "-" && &dt[4..5] == "-" 
                    => match NaiveDateTime::parse_from_str(dt, "%m-%d-%y %H:%M") {
                        Ok(date) => return CSVType::DateTime(date),
                        Err(_) => return self.convert_to_string(),
                    },
                    // mm/d/yy hh:mm
                    dt if &dt[10..11] == ":"
                    && &dt[2..3] == "-" && &dt[4..5] == "-" 
                    => match NaiveDateTime::parse_from_str(dt, "%m-%d-%y %H:%M") {
                        Ok(date) => return CSVType::DateTime(date),
                        // dd/m/yy hh:mm
                        Err(_) => match NaiveDateTime::parse_from_str(dt, "%d-%m-%y %H:%M") {
                            Ok(date) => return CSVType::DateTime(date),
                            // yy/m/dd hh:mm
                            Err(_) => match NaiveDateTime::parse_from_str(dt, "%y-%m-%d %H:%M") {
                                Ok(date) => return CSVType::DateTime(date),
                                // yy/d/mm hh:mm
                                Err(_) => match NaiveDateTime::parse_from_str(dt, "%y-%d-%m %H:%M") {
                                    Ok(date) => return CSVType::DateTime(date),
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
                        Ok(date) => return CSVType::DateTime(date),
                        Err(_) => return self.convert_to_string(),
                    },
                    // mm/d/yy h:mm
                    dt if &dt[9..10] == ":"
                    && &dt[2..3] == "-" && &dt[4..5] == "-" 
                    => match NaiveDateTime::parse_from_str(dt, "%m-%d-%y %H:%M") {
                        Ok(date) => return CSVType::DateTime(date),
                        // dd/m/yy h:mm
                        Err(_) => match NaiveDateTime::parse_from_str(dt, "%d-%m-%y %H:%M") {
                            Ok(date) => return CSVType::DateTime(date),
                            // yy/m/dd h:mm
                            Err(_) => match NaiveDateTime::parse_from_str(dt, "%y-%m-%d %H:%M") {
                                Ok(date) => return CSVType::DateTime(date),
                                // yy/d/mm h:mm
                                Err(_) => match NaiveDateTime::parse_from_str(dt, "%y-%d-%m %H:%M") {
                                    Ok(date) => return CSVType::DateTime(date),
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
}