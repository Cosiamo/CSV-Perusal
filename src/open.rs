use crate::csvtype::{CSVType, ByteString};
use std::{fs::File, path::Path, io::BufReader};

pub fn open_csv(path: &str) -> Result<Vec<Vec<CSVType>>, csv::Error> {
    let mut outer_vec: Vec<Vec<CSVType>> = Vec::new();

    match csv_read(path) {
        Ok(data) => {
            for y in 0..data.len() {
                let mut inner_vec: Vec<CSVType> = Vec::new();
                for (_x, bytes) in data[y].iter().enumerate() {
                    match bytes {
                        [] => inner_vec.push(CSVType::Empty), 
                        _ => {
                            let t = String::from_utf8_lossy(&bytes);
                            let s = t.replace(|c: char| !c.is_ascii(), "");
                            match s.trim().parse::<i64>() {
                                Ok(v) => inner_vec.push(CSVType::Int(v)),
                                Err(_) => match s.trim().parse::<f64>() {
                                    Ok(v) => inner_vec.push(CSVType::Float(v)),
                                    Err(_) => inner_vec.push(match_catch(s))
                                },
                            }
                        }
                    }
                }
                outer_vec.push(inner_vec)
            }
        },
        Err(e) => return Err(e),
    };

    Ok(outer_vec)
}

fn csv_read(path: &str) -> Result<Vec<csv::ByteRecord>, csv::Error> {
    match File::open(Path::new(&path)) {
        Ok(file_handle) => {
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
            return Ok(data)
        },
        Err(e) => return Err(e.into()),
    };
}

fn match_catch(s: String) -> CSVType {
    let bytestring = ByteString {s};

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
            // checks for time 24h
            bs if bs.is_time_24h() => return bs.time_match(),
            // checks for date
            bs if bs.is_date() => return bs.date_match(),
            // catch
            _ => return bs.convert_to_string(),
        },
        bs if bs.contains_number()
        => match bs {
            // checks for time 12h
            bs if bs.is_time_12h() => return bs.time_match(),
            // checks datetime
            bs if bs.is_datetime() => return bs.datetime_match(),
            // checks for date
            bs if bs.is_date_w_abbrv() => return bs.date_w_abbrv_match(),
            _ => return bs.convert_to_string(),
        },
        _ => return bytestring.convert_to_string(),
    }
}
