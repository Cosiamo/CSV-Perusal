use crate::{types::{Byte, ByteString}, CSVType};

impl<'slice> Byte<'slice> {
    pub fn num_match(&self) -> CSVType {
        let cow = String::from_utf8_lossy(&self.byte);
        let string = cow.replace(|c: char| !c.is_ascii(), "");
        match string.trim().parse::<i64>() {
            Ok(v) => CSVType::Int(v),
            Err(_) => match string.trim().parse::<f64>() {
                Ok(v) => CSVType::Float(v),
                Err(_) => self.catch()
            },
        }
    }

    pub fn catch(&self) -> CSVType {
        match_catch(&self.byte)
    }
}

pub fn match_catch(bytes: &[u8]) -> CSVType {
    let cow = String::from_utf8_lossy(&bytes);
    let string = cow.replace(|c: char| !c.is_ascii(), "");
    let bytestring = ByteString {bytestring: string};

    match bytestring {
        bs if bs.is_empty() => return CSVType::Empty,
        bs if !bs.contains_number()
        => return bs.convert_to_string(),
        bs if bs.trimmed().chars().all(char::is_numeric)
        => match bs {
            // checks for negative percent
            bs if bs.is_percent_neg()
            => match bs.remove_symbol().parse::<f64>() {
                Ok(v) => return CSVType::Float(v),
                Err(_) => return bs.convert_to_string(),
            },
            // checks for positive percent
            bs if bs.is_percent_pos()
            => match bs.remove_symbol().parse::<f64>() {
                Ok(v) => return CSVType::Float(v),
                Err(_) => return bs.convert_to_string(),
            },
            // checks for negative currency
            bs if bs.is_currency_neg()
            => match bs.remove_symbol().parse::<f64>() {
                Ok(v) => return CSVType::Float(v),
                Err(_) => return bs.convert_to_string(),
            },
            // checks for positive currency 
            bs if bs.is_currency_pos()
            => match bs.remove_symbol().parse::<f64>() {
                Ok(v) => return CSVType::Float(v),
                Err(_) => return bs.convert_to_string(),
            },
            _ => return bs.convert_to_string(),
        },
        bs if bs.contains_number()
        => match bs {
            bs if bs.is_date_w_abbrv() => return bs.date_w_abbrv_match(),
            _ => return bs.convert_to_string(),
        },
        _ => return bytestring.convert_to_string(),
    }
}