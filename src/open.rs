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
                                Ok(s) => match s.parse::<i64>() {
                                    Ok(v) => innervec.push(CSVType::Int(v)),
                                    Err(_) => match s.parse::<f64>() {
                                        Ok(v) => innervec.push(CSVType::Float(v)),
                                        Err(_) => innervec.push(match_catch(s))
                                    },
                                },
                                Err(_) => {
                                    let t = String::from_utf8_lossy(&bytes);
                                    let s = t.replace(|c: char| !c.is_ascii(), "");
                                    match s.parse::<i64>() {
                                        Ok(v) => innervec.push(CSVType::Int(v)),
                                        Err(_) => match s.parse::<f64>() {
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
    match bytestring.trimmed().len() {
        // checks for "\u{1680}" (Ogham Space Mark) 
        0 => match bytestring.s.parse::<String>() {
            Ok(s) => return CSVType::String(s),
            Err(e) => return CSVType::Error(e),
        },
        _ => match bytestring.trimmed().chars().all(char::is_numeric) {
            true => match bytestring {
                // checks for time
                bs if bs.is_time() => return bs.time_match(),
                // checks for date
                bs if bs.is_date() => return bs.date_match(),
                // checks for positive percent
                bs if bs.is_percent_pos()
                => match bs {
                    bs if bs.s.contains(".")
                    => match bs.trimmed().parse::<f64>() {
                        Ok(v) => return CSVType::Float(v / 100.0),
                        Err(_) => match bs.s.parse::<String>() {
                            Ok(s) => return CSVType::String(s),
                            Err(e) => return CSVType::Error(e),
                        },
                    },
                    _ => match bs.trimmed().parse::<f64>() {
                        Ok(v) => return CSVType::Float(v),
                        Err(_) => match bs.s.parse::<String>() {
                            Ok(s) => return CSVType::String(s),
                            Err(e) => return CSVType::Error(e),
                        },
                    },
                },
                // checks for negative percent
                bs if bs.is_percent_neg()
                => match bs {
                    bs if bs.s.contains(".")
                    => match bs.trimmed().parse::<f64>() {
                        Ok(v) => return CSVType::Float((v * -1.0) / 100.0),
                        Err(_) => match bs.s.parse::<String>() {
                            Ok(s) => return CSVType::String(s),
                            Err(e) => return CSVType::Error(e),
                        },
                    },
                    _ => match bs.trimmed().parse::<f64>() {
                        Ok(v) => return CSVType::Float(v * -1.0),
                        Err(_) => match bs.s.parse::<String>() {
                            Ok(s) => return CSVType::String(s),
                            Err(e) => return CSVType::Error(e),
                        },
                    },
                },
                // checks for positive currency 
                bs if bs.is_currency_pos()
                => match bs {
                    bs if bs.s.contains(".")
                    => match bs.trimmed().parse::<f64>() {
                        Ok(v) => return CSVType::Float(v / 100.0),
                        Err(_) => match bs.s.parse::<String>() {
                            Ok(s) => return CSVType::String(s),
                            Err(e) => return CSVType::Error(e),
                        },
                    },
                    _ => match bs.trimmed().parse::<f64>() {
                        Ok(v) => return CSVType::Float(v),
                        Err(_) => match bs.s.parse::<String>() {
                            Ok(s) => return CSVType::String(s),
                            Err(e) => return CSVType::Error(e),
                        },
                    },
                },
                // checks for negative currency
                bs if bs.is_currency_neg()
                => match bs {
                    bs if bs.s.contains(".")
                    => match bs.trimmed().parse::<f64>() {
                        Ok(v) => return CSVType::Float((v * -1.0) / 100.0),
                        Err(_) => match bs.s.parse::<String>() {
                            Ok(s) => return CSVType::String(s),
                            Err(e) => return CSVType::Error(e),
                        },
                    },
                    _ => match bs.trimmed().parse::<f64>() {
                        Ok(v) => return CSVType::Float(v * -1.0),
                        Err(_) => match bs.s.parse::<String>() {
                            Ok(s) => return CSVType::String(s),
                            Err(e) => return CSVType::Error(e),
                        },
                    },
                },
                // catch
                _ => match bytestring.s.parse::<i64>() {
                    Ok(v) => return CSVType::Int(v),
                    Err(_) => match bytestring.s.parse::<f64>() {
                        Ok(v) => return CSVType::Float(v),
                        Err(_) => match bytestring.s.parse::<String>() {
                            Ok(s) => {return CSVType::String(s)},
                            Err(e) => return CSVType::Error(e),
                        },
                    }
                },
            },
            false => match bytestring {
                bs if bs.is_datetime() => return bs.datetime_match(),
                bs if bs.is_time_12h() => return bs.time_match(),
                _ => match bytestring.s.parse::<String>() {
                    Ok(s) => return CSVType::String(s),
                    Err(e) => return CSVType::Error(e),
                },
            }
        },
    }
}
