use regex::bytes::Regex;
use once_cell::sync::Lazy;

use crate::types::{Byte, ByteString};

// is_date
static DATE: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[0-3]?[0-9].[0-3]?[0-9].(?:[0-9]{2})?[0-9]{2}$").unwrap()
});
// is_datetime
static DT12: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[0-3]?[0-9].[0-3]?[0-9].(?:[0-9]{2})?[0-9]{2} ([1-9]|0[1-9]|1[0-2]):[0-5][0-9] ([AaPp][Mm])$").unwrap()
});
static DT12_W_S: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[0-3]?[0-9].[0-3]?[0-9].(?:[0-9]{2})?[0-9]{2} ([1-9]|0[1-9]|1[0-2]):[0-5][0-9]:[0-5][0-9] ([AaPp][Mm])$").unwrap()
});
static DT24: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[0-3]?[0-9].[0-3]?[0-9].(?:[0-9]{2})?[0-9]{2} ([01]?[0-9]|2[0-3]):[0-5][0-9]$").unwrap()
});
static DT24_W_S: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[0-3]?[0-9].[0-3]?[0-9].(?:[0-9]{2})?[0-9]{2} ([01]?[0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$").unwrap()
});
// is_time_24h
static H24: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^([01]?[0-9]|2[0-3]):[0-5][0-9]$").unwrap()
});
static H24_W_S: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^([01]?[0-9]|2[0-3]):[0-5][0-9]:[0-5][0-9]$").unwrap()
});
// is_time_12h
static H12: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^([1-9]|0[1-9]|1[0-2]):[0-5][0-9] ([AaPp][Mm])$").unwrap()
});
static H12_W_S: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^([1-9]|0[1-9]|1[0-2]):[0-5][0-9]:[0-5][0-9] ([AaPp][Mm])$").unwrap()
});
// is_number
static NUM: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[-+]?\d*\.?\d*(\d+[eE][-+]?)?\d+$").unwrap()
});

impl<'slice> Byte<'slice> {
    pub fn is_date(&self) -> bool {
        DATE.captures(&self.b).is_some()
    }

    pub fn is_datetime(&self) -> bool {
        match self.am_pm() {
            true => match DT12.captures(&self.b) {
                Some(_) => true,
                None => DT12_W_S.captures(&self.b).is_some(),
            },
            false => match DT24.captures(&self.b) {
                Some(_) => true,
                None => DT24_W_S.captures(&self.b).is_some(),
            }
        }
    }

    pub fn is_time_24h(&self) -> bool {
        match H24.captures(&self.b) {
            Some(_) => true,
            None => H24_W_S.captures(&self.b).is_some(),
        }
    }

    pub fn is_time_12h(&self) -> bool {
        match self.am_pm() {
            true => match H12.captures(&self.b) {
                Some(_) => true,
                None => H12_W_S.captures(&self.b).is_some(),
            },
            false => false,
        }
    }

    pub fn is_number(&self) -> bool {
        NUM.captures(&self.b).is_some()
    }
}

// is_date_w_abbrv
static DATE_W_ABBRV: Lazy<regex::Regex> = Lazy::new(|| {
    regex::Regex::new(r"^\d{2}-(JAN|FEB|MAR|APR|MAY|JUN|JUL|AUG|SEP|OCT|NOV|DEC)-\d{4}$").unwrap()
});

impl ByteString {
    pub fn is_date_w_abbrv(&self) -> bool {
        match self.s.trim().to_ascii_uppercase() {
            s if s.contains("-")
            || s.contains("/")
            => DATE_W_ABBRV.captures(&s).is_some(),
            _ => false,
        }
    }
}