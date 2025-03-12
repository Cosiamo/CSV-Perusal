use regex::bytes::Regex;
use once_cell::sync::Lazy;

use crate::types::{Byte, ByteString};

// is_number
static NUM: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[-+]?\d*\.?\d*(\d+[eE][-+]?)?\d+$").unwrap()
});

impl<'slice> Byte<'slice> {
    pub fn is_number(&self) -> bool {
        NUM.captures(&self.byte).is_some()
    }
}

// is_date_w_abbrv
static DATE_W_ABBRV: Lazy<regex::Regex> = Lazy::new(|| {
    regex::Regex::new(r"^\d{2}-(JAN|FEB|MAR|APR|MAY|JUN|JUL|AUG|SEP|OCT|NOV|DEC)-\d{4}$").unwrap()
});

impl ByteString {
    pub fn is_date_w_abbrv(&self) -> bool {
        match self.bytestring.trim().to_ascii_uppercase() {
            string if string.contains("-")
            || string.contains("/")
            => DATE_W_ABBRV.captures(&string).is_some(),
            _ => false,
        }
    }
}