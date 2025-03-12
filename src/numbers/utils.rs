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
        match self {
            bs if bs.is_empty() => return CSVType::Empty,
            bs if !bs.contains_number() => return bs.convert_to_string(),
            bs if bs.trimmed().chars().all(char::is_numeric) => match bs {
                // checks for negative percent
                bs if bs.is_percent_neg()
                => match bs.remove_symbol().parse::<f64>() {
                    Ok(val) => {
                        let res = val / 100.0;
                        return CSVType::Float(res)
                    },
                    Err(_) => return bs.convert_to_string(),
                },
                // checks for positive percent
                bs if bs.is_percent_pos()
                => match bs.remove_symbol().parse::<f64>() {
                    Ok(val) => {
                        let res = val / 100.0;
                        return CSVType::Float(res)
                    },
                    Err(_) => return bs.convert_to_string(),
                },
                // checks for negative currency
                bs if bs.is_currency_neg()
                => match bs.remove_symbol().parse::<f64>() {
                    Ok(val) => return CSVType::Float(val),
                    Err(_) => return bs.convert_to_string(),
                },
                // checks for positive currency 
                bs if bs.is_currency_pos()
                => match bs.remove_symbol().parse::<f64>() {
                    Ok(val) => return CSVType::Float(val),
                    Err(_) => return bs.convert_to_string(),
                },
                _ => return bs.convert_to_string(),
            },
            bs if bs.contains_number() => match bs {
                bs if bs.is_date_w_abbrv() => return bs.date_w_abbrv_match(),
                _ => return bs.convert_to_string(),
            },
            _ => return self.convert_to_string(),
        }
    }
}