use crate::{types::ByteString, CSVType};

impl ByteString {
    pub fn num_match(&self) -> CSVType {
        match self.bytestring.trim().parse::<i64>() {
            Ok(v) => CSVType::Int(v),
            Err(_) => match self.bytestring.trim().parse::<f64>() {
                Ok(v) => CSVType::Float(v),
                Err(_) => self.catch()
            },
        }
    }

    pub fn catch(&self) -> CSVType {
        if self.is_empty() {
            return CSVType::Empty
        }
        if !self.contains_number() {
            return self.convert_to_string()
        }
        if self.is_numeric_with_symbols() && self.is_percentage() {
            if let Ok(val) = self.remove_symbol().parse::<f64>() {
                let res = val / 100.0;
                return CSVType::Float(res)
            }
        }
        if self.is_numeric_with_symbols() && self.is_currency() {
            if let Ok(val) = self.remove_symbol().parse::<f64>() {
                return CSVType::Float(val)
            }
        }
        if self.contains_number() && self.is_date_w_abbrv() {
            return self.date_w_abbrv_match()
        }
        return self.convert_to_string()
    }

    pub fn is_numeric_with_symbols(&self) -> bool {
        self.trimmed().chars().all(char::is_numeric)
    }

    pub fn is_percentage(&self) -> bool {
        self.is_percent_pos() || self.is_percent_neg()
    }

    pub fn is_currency(&self) -> bool {
        self.is_currency_pos() || self.is_currency_neg()
    }
}