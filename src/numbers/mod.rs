use crate::{error::CellError, types::ByteString, CSVType};

pub mod currency;
pub mod percent;
pub mod utils;

impl ByteString {
    pub fn trimmed(&self) -> String {
        return self.bytestring
            .trim()
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
        return self.bytestring
            .trim()
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
        match self.bytestring.trim().parse::<String>() {
            Ok(string) => return CSVType::String(string),
            Err(_) => return CSVType::Error(CellError::DataParseError),
        }
    }
    
    pub fn is_empty(&self) -> bool {
        match self.bytestring.trim().len() {
            0 => true,
            _ => false,
        }
    }

    // false num catch matches
    pub fn contains_number(&self) -> bool {
        let vector = self.bytestring.trim().chars().map(|char| 
            match char {
                local_char if local_char.is_numeric() => true,
                _ => false,
            }
        ).collect::<Vec<bool>>();

        match vector {
            local_vector if local_vector.contains(&true) => true,
            _ => false,
        }
    }
}