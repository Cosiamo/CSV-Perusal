use crate::{utils::ByteString, csvtype::{CSVType, CSVTypeError}};
use std::{fs::File, path::Path, io::BufReader};

pub fn open_csv(path: &str) -> Vec<CSVType> {
    let mut datatype: Vec<CSVType> = Vec::new();

    match csv_read(path) {
        Ok(data) => {
            for y in 0..data.len() {
                for (_x, bytes) in data[y].iter().enumerate() {
                    match bytes {
                        [] => datatype.push(CSVType::Empty), 
                        _ => {
                            match String::from_utf8((&bytes).to_vec()) {
                                Ok(s) => match s.parse::<i64>() {
                                    Ok(v) => datatype.push(CSVType::Int(v)),
                                    Err(_) => match s.parse::<f64>() {
                                        Ok(v) => datatype.push(CSVType::Float(v)),
                                        Err(_) => datatype.push(match_catch(s))
                                    },
                                },
                                Err(_) => {
                                    let t = String::from_utf8_lossy(&bytes);
                                    let s = t.replace(|c: char| !c.is_ascii(), "");
                                    match s.parse::<i64>() {
                                        Ok(v) => datatype.push(CSVType::Int(v)),
                                        Err(_) => match s.parse::<f64>() {
                                            Ok(v) => datatype.push(CSVType::Float(v)),
                                            Err(_) => datatype.push(match_catch(s))
                                        },
                                    }
                                },
                            }
                        }
                    }
                }
            }
        },
        Err(e) => datatype.push(CSVType::Error(CSVTypeError::ByteError(e))),
    };

    return datatype;
}

fn csv_read(path: &str) -> Result<Vec<csv::ByteRecord>, csv::Error> {
    let file_handle = match File::open(Path::new(&path)) {
        Ok(val) => val,
        Err(e) => panic!("\u{1b}[31m{:?}\u{1b}[39m", e),
    };
    let reader = BufReader::new(file_handle);
    let mut data: Vec<csv::ByteRecord> = Vec::new();
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(reader);
    for result in rdr.byte_records() {
        match result {
            Ok(record) => data.push(record),
            Err(e) => return Err(e),
        };
    }
    
    return Ok(data);
}

fn match_catch(s: String) -> CSVType {
    let new_str = ByteString {s};
    match new_str.trimmed().len() {
        // checks for "\u{1680}" (Ogham Space Mark) 
        0 => return CSVType::String(new_str.s),
        _ => match new_str.trimmed().chars().all(char::is_numeric) {
            true => match new_str {
                // checks for positive percent
                bs if bs.is_percent_pos()
                => match bs.trimmed().parse::<f64>() {
                    Ok(v) => return CSVType::Float(v / 100.0),
                    Err(_) => match bs.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(CSVTypeError::Parse(e)),
                    },
                },
                // checks for negative percent
                bs if bs.is_percent_neg()
                => match bs.trimmed().parse::<f64>() {
                    Ok(v) => return CSVType::Float((v * -1.0) / 100.0),
                    Err(_) => match bs.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(CSVTypeError::Parse(e)),
                    },
                },
                // checks for date
                bs if bs.is_date() => return bs.date_match(),
                // checks for positive currency 
                bs if bs.is_currency_pos()
                => match bs.trimmed().parse::<f64>() {
                    Ok(v) => return CSVType::Float(v / 100.0),
                    Err(_) => match bs.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(CSVTypeError::Parse(e)),
                    },
                },
                // checks for negative currency
                bs if bs.is_currency_neg()
                => match bs.trimmed().parse::<f64>() {
                    Ok(v) => return CSVType::Float((v * -1.0) / 100.0),
                    Err(_) => match bs.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(CSVTypeError::Parse(e)),
                    },
                },
                // checks if negative integer 
                bs if bs.is_int_neg() 
                => match bs.trimmed().parse::<i64>() {
                    Ok(v) => return CSVType::Int(v * -1),
                    Err(_) => match bs.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(CSVTypeError::Parse(e)),
                    },
                },
                // catches positive integers
                _ => match new_str.s.parse::<i64>() {
                    Ok(v) => return CSVType::Int(v),
                    Err(_) => match new_str.s.parse::<String>() {
                        Ok(s) => return CSVType::String(s),
                        Err(e) => return CSVType::Error(CSVTypeError::Parse(e)),
                    },
                },
            },
            false => match new_str.s.parse::<String>() {
                Ok(s) => return CSVType::String(s),
                Err(e) => return CSVType::Error(CSVTypeError::Parse(e)),
            },
        },
    }
}
