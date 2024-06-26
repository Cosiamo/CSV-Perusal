use crate::{error::CellError, types::ByteString, CSVType};

pub mod currency;
pub mod percent;
pub mod utils;

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
            Err(_) => return CSVType::Error(CellError::DataParseError),
        }
    }
    
    pub fn is_empty(&self) -> bool {
        match self.s.trim().len() {
            0 => true,
            _ => false,
        }
    }

    // false num catch matches
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
}