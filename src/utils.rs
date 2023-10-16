use crate::csvtype::{ByteString, CSVType};
use regex::Regex;

impl ByteString {
    pub fn trimmed(&self) -> String {
        return self.s.trim()
        .replace("%", "")
        .replace(",", "")
        .replace(".", "")
        .replace("-", "")
        .replace("/", "")
        .replace(":", "")
        .replace("$", "")
        .replace("€", "")
        .replace("£", "")
        .replace("¥", "")
        .replace("₣", "")
        .replace("₹", "")
        .replace("د.ك", "")
    }

    pub fn remove_symbol(&self) -> String {
        return self.s.trim()
        .replace(",", "")
        .replace("%", "")
        .replace("$", "")
        .replace("€", "")
        .replace("£", "")
        .replace("¥", "")
        .replace("₣", "")
        .replace("₹", "")
        .replace("د.ك", "")
    }

    pub fn convert_to_string(&self) -> CSVType {
        match self.s.trim().parse::<String>() {
            Ok(s) => return CSVType::String(s),
            Err(_) => return CSVType::Error("PARSING ERROR: Error parsing string".to_string()),
        }
    }
}

// all num matches
impl ByteString {
    pub fn is_date(&self) -> bool {
        match self.s.trim() {
            s if s.contains("-")
            || s.contains("/")
            => {
                let date_regex = Regex::new(r"^[0-3]?[0-9].[0-3]?[0-9].(?:[0-9]{2})?[0-9]{2}$").unwrap();
                match date_regex.captures(&s).is_some() {
                    true => true,
                    false => false,
                }
            },
            _ => false,
        }
    }

    pub fn is_time_24h(&self) -> bool {
        match self.s.trim() {
            s if s.contains(":")
            => {
                let hours_mins = Regex::new(r"^([01]?[0-9]|2[0-3]):[0-5][0-9]$").unwrap();
                match hours_mins.captures(s) {
                    Some(_) => true,
                    None => {
                        let hours_mins_secs = Regex::new(r"^([01]?[0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$").unwrap();
                        match hours_mins_secs.captures(s) {
                            Some(_) => true,
                            None => false,
                        }
                    },
                }
            },
            _ => false,
        }
    }

    pub fn is_percent_pos(&self) -> bool {
        match self.s.trim() {
            s if s.contains("%")
            => match s.chars().map(|x| x).collect::<Vec<char>>() {
                chars if chars.contains(&"%".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                => true,
                chars if chars.contains(&"%".chars().map(|x| x).collect::<Vec<char>>()[0])
                && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn is_percent_neg(&self) -> bool {
        match self.s.trim() {
            s if s.contains("%")
            && s.contains("-")
            => match s.chars().map(|x| x).collect::<Vec<char>>() {
                chars if chars.contains(&"%".chars().map(|x| x).collect::<Vec<char>>()[0])
                && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                => true,
                chars if chars.contains(&"%".chars().map(|x| x).collect::<Vec<char>>()[0])
                && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                => true,
                _ => false,
            },
            _ => false,
        }
    }

    pub fn is_currency_pos(&self) -> bool {
        match self.s.trim() {
            s if s.contains("$")
            || s.contains("€")
            || s.contains("£")
            || s.contains("¥")
            || s.contains("₣")
            || s.contains("₹")
            || s.contains("د.ك")
            => {
                match s.chars().map(|x| x).collect::<Vec<char>>() {
                    // $
                    chars if chars[0] == "$".chars().map(|x| x).collect::<Vec<char>>()[0] 
                    => true,
                    chars if chars[0] == "$".chars().map(|x| x).collect::<Vec<char>>()[0] 
                    && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    => true,
                    // €
                    chars if chars.contains(&"€".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    => true,
                    chars if chars.contains(&"€".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    => true,
                    // £
                    chars if chars.contains(&"£".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    => true,
                    chars if chars.contains(&"£".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    => true,
                    // ¥
                    chars if chars.contains(&"¥".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    => true,
                    chars if chars.contains(&"¥".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    => true,
                    // ₣
                    chars if chars.contains(&"₣".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    => true,
                    chars if chars.contains(&"₣".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    => true,
                    // ₹
                    chars if chars.contains(&"₹".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    => true,
                    chars if chars.contains(&"₹".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0])
                    => true,
                    // د.ك
                    chars if chars.contains(&"د.ك".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    => true,
                    chars if chars.contains(&"د.ك".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    => true,
                    _ => false,
                }
            },
            _ => false,
        }
    }

    pub fn is_currency_neg(&self) -> bool {
        match self.s.trim() {
            s if s.contains("-")
            && s.contains("$")
            || s.contains("€")
            || s.contains("£")
            || s.contains("¥")
            || s.contains("₣")
            || s.contains("₹")
            || s.contains("د.ك")
            => {
                match s.chars().map(|x| x).collect::<Vec<char>>() {
                    // $
                    chars if chars.contains(&"$".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    || chars[1] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    => true,
                    chars if chars.contains(&"$".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    || chars[1] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    => true,
                    // €
                    chars if chars.contains(&"€".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    || chars[1] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    => true,
                    chars if chars.contains(&"€".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    || chars[1] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    => true,
                    // £
                    chars if chars.contains(&"£".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    || chars[1] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    => true,
                    chars if chars.contains(&"£".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    || chars[1] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    => true,
                    // ¥
                    chars if chars.contains(&"¥".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0]
                    || chars[1] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    => true,
                    chars if chars.contains(&"¥".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    || chars[1] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    => true,
                    // ₣
                    chars if chars.contains(&"₣".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0]
                    || chars[1] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    => true,
                    chars if chars.contains(&"₣".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    || chars[1] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    => true,
                    // ₹
                    chars if chars.contains(&"₹".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    || chars[1] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    => true,
                    chars if chars.contains(&"₹".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    || chars[1] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    => true,
                    // د.ك
                    chars if chars.contains(&"د.ك".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    || chars[1] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    => true,
                    chars if chars.contains(&"د.ك".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars.contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) 
                    && chars[0] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    || chars[1] == "-".chars().map(|x| x).collect::<Vec<_>>()[0] 
                    => true,
                    _ => false,
                }
            },
            _ => false,
        }
    }
}

// false num catch matches
impl ByteString {
    pub fn is_empty(&self) -> bool {
        match self.s.trim().len() {
            0 => true,
            _ => false,
        }
    }

    pub fn contains_number(&self) -> bool {
        let v = self.s.trim().chars().map(|x| 
            match x {
                x if x.is_numeric() => true,
                _ => false,
            }
        ).collect::<Vec<bool>>();

        match v {
            v if v.contains(&true) => true,
            _ => false,
        }
    }

    pub fn is_time_12h(&self) -> bool {
        match self.s.trim() {
            s if s.contains(":")
            && s.to_ascii_uppercase().contains("AM")
            || s.to_ascii_uppercase().contains("PM")
            => {
                let hours_mins = Regex::new(r"^([1-9]|0[1-9]|1[0-2]):[0-5][0-9] ([AaPp][Mm])$").unwrap();
                match hours_mins.captures(s) {
                    Some(_) => true,
                    None => {
                        let hours_mins_secs = Regex::new(r"^([1-9]|0[1-9]|1[0-2]):[0-5][0-9]:[0-5][0-9] ([AaPp][Mm])$").unwrap();
                        match hours_mins_secs.captures(s) {
                            Some(_) => true,
                            None => false
                        }
                    },
                }
            },
            _ => false,
        }
    }

    pub fn is_date_w_abbrv(&self) -> bool {
        match self.s.trim().to_ascii_uppercase() {
            s if s.contains("-")
            || s.contains("/")
            => {
                let date_regex1 = Regex::new(r"^\d{2}-(JAN|FEB|MAR|APR|MAY|JUN|JUL|AUG|SEP|OCT|NOV|DEC)-\d{4}$").unwrap();
                    match date_regex1.captures(&s).is_some() {
                        true => true,
                        false => false,
                    }
            },
            _ => false,
        }
    }

    pub fn is_datetime(&self) -> bool {
        match self.s.trim() {
            s if s.contains(":")
            && s.contains("-")
            || s.contains("/")
            => {
                match s {
                    s if s.to_ascii_uppercase().contains("AM")
                    || s.to_ascii_uppercase().contains("PM") 
                    => {
                        let h12 = Regex::new(r"^[0-3]?[0-9].[0-3]?[0-9].(?:[0-9]{2})?[0-9]{2} ([1-9]|0[1-9]|1[0-2]):[0-5][0-9] ([AaPp][Mm])$").unwrap();
                        match h12.captures(s) {
                            Some(_) => true,
                            None => {
                                let h12_w_sec = Regex::new(r"^[0-3]?[0-9].[0-3]?[0-9].(?:[0-9]{2})?[0-9]{2} ([1-9]|0[1-9]|1[0-2]):[0-5][0-9]:[0-5][0-9] ([AaPp][Mm])$").unwrap();
                                match h12_w_sec.captures(s) {
                                    Some(_) => true,
                                    None => false,
                                }
                            },
                        }
                    },
                    _ => {
                        let h24 = Regex::new(r"^[0-3]?[0-9].[0-3]?[0-9].(?:[0-9]{2})?[0-9]{2} ([01]?[0-9]|2[0-3]):[0-5][0-9]$").unwrap();
                        match h24.captures(s) {
                            Some(_) => true,
                            None => {
                                let h24_w_sec = Regex::new(r"^[0-3]?[0-9].[0-3]?[0-9].(?:[0-9]{2})?[0-9]{2} ([01]?[0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$").unwrap();
                                match h24_w_sec.captures(s) {
                                    Some(_) => true,
                                    None => false,
                                }
                            },
                        }
                    },
                }
            },
            _ => false,
        }
    }
}