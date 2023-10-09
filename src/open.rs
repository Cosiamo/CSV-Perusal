use crate::csvtype::{CSVType, ByteString};
use std::{fs::File, path::Path, io::BufReader};

pub fn open_csv(path: &str) -> Result<Vec<Vec<CSVType>>, csv::Error> {
    let mut outtervec: Vec<Vec<CSVType>> = Vec::new();

    match csv_read(path) {
        Ok(data) => {
            for y in 0..data.len() {
                let mut innervec: Vec<CSVType> = Vec::new();
                for (_x, bytes) in data[y].iter().enumerate() {
                    match bytes {
                        [] => innervec.push(CSVType::Empty), 
                        _ => {
                            match String::from_utf8((&bytes).to_vec()) {
                                Ok(s) => match s.trim().parse::<i64>() {
                                    Ok(v) => innervec.push(CSVType::Int(v)),
                                    Err(_) => match s.trim().parse::<f64>() {
                                        Ok(v) => innervec.push(CSVType::Float(v)),
                                        Err(_) => innervec.push(match_catch(s))
                                    },
                                },
                                Err(_) => {
                                    let t = String::from_utf8_lossy(&bytes);
                                    let s = t.replace(|c: char| !c.is_ascii(), "");
                                    match s.trim().parse::<i64>() {
                                        Ok(v) => innervec.push(CSVType::Int(v)),
                                        Err(_) => match s.trim().parse::<f64>() {
                                            Ok(v) => innervec.push(CSVType::Float(v)),
                                            Err(_) => innervec.push(match_catch(s))
                                        },
                                    }
                                },
                            }
                        }
                    }
                }
                outtervec.push(innervec)
            }
        },
        Err(e) => return Err(e),
    };

    Ok(outtervec)
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
        bs if !bs.contains_number()
        => match bs.s.parse::<String>() {
            Ok(s) => return CSVType::String(s),
            Err(e) => return CSVType::Error(e),
        },
        bs if bs.trimmed().chars().all(char::is_numeric)
        => match bs {
            // checks for negative percent
            bs if bs.is_percent_neg()
            => match bs.remove_symbol().parse::<f64>() {
                Ok(v) => return CSVType::Float(v),
                Err(_) => match bs.s.parse::<String>() {
                    Ok(s) => return CSVType::String(s),
                    Err(e) => return CSVType::Error(e),
                },
            },
            // checks for positive percent
            bs if bs.is_percent_pos()
            => match bs.remove_symbol().parse::<f64>() {
                Ok(v) => return CSVType::Float(v),
                Err(_) => match bs.s.parse::<String>() {
                    Ok(s) => return CSVType::String(s),
                    Err(e) => return CSVType::Error(e),
                },
            },
            // checks for negative currency
            bs if bs.is_currency_neg()
            => match bs.remove_symbol().parse::<f64>() {
                Ok(v) => return CSVType::Float(v),
                Err(_) => match bs.s.parse::<String>() {
                    Ok(s) => return CSVType::String(s),
                    Err(e) => return CSVType::Error(e),
                },
            },
            // checks for positive currency 
            bs if bs.is_currency_pos()
            => match bs.remove_symbol().parse::<f64>() {
                Ok(v) => return CSVType::Float(v),
                Err(_) => match bs.s.parse::<String>() {
                    Ok(s) => return CSVType::String(s),
                    Err(e) => return CSVType::Error(e),
                },
            },
            // checks for time
            bs if bs.is_time_24h() => return bs.time_match(),
            // checks for date
            bs if bs.is_date() => return bs.date_match(),
            // catch
            _ => match bs.s.parse::<String>() {
                Ok(s) => {return CSVType::String(s)},
                Err(e) => return CSVType::Error(e),
            },
        },
        bs if bs.contains_number()
        => match bs {
            bs if bs.is_time_12h() => return bs.time_match(),
            bs if bs.is_datetime() => return bs.datetime_match(),
            bs => match bs.s.parse::<String>() {
                Ok(s) => return CSVType::String(s),
                Err(e) => return CSVType::Error(e),
            },
        },
        _ => match bytestring.s.parse::<String>() {
            Ok(s) => return CSVType::String(s),
            Err(e) => return CSVType::Error(e),
        },
    }
}
