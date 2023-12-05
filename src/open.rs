use crate::{csvtype::{CSVType, Byte}, utils::match_catch};
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
                            let byte = Byte{b: bytes};
                            match byte {
                                byte if byte.is_number() => inner_vec.push(byte.num_match()),
                                byte if byte.is_dt()=> inner_vec.push(byte.date_and_time()),
                                _ => inner_vec.push(match_catch(bytes)),
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