use crate::{errors::CSVPerusalError, numbers::utils::match_catch, types::Byte, CSVType};
use std::{fs::File, path::Path, io::BufReader};
use csv::ByteRecord;
use rayon::prelude::*;
use std::sync::mpsc::channel;
use itertools::Itertools;

/// Assigns the bytes from a vector of [`csv::ByteRecord`] to a specific [`CSVType`].
/// 
/// Uses parallel iteration via [`rayon`] to double the speed of iteration of the input data.
pub fn assign_bytes(mut data: Vec<ByteRecord>) -> Result<Vec<Vec<CSVType>>, CSVPerusalError> {
    let (sender, receiver) = channel();
    data.par_iter_mut().enumerate().for_each_with(sender, |send, (idx, item)| {
        let mut inner_vec: Vec<CSVType> = Vec::new();
        item.iter().for_each(|bytes: &[u8]| {
            match bytes {
                [] => inner_vec.push(CSVType::Empty),
                _ => {
                    let byte: Byte<'_> = Byte{byte: bytes};
                    match byte {
                        byte if byte.is_number() => inner_vec.push(byte.num_match()),
                        byte if byte.is_dt()=> inner_vec.push(byte.date_and_time()),
                        _ => inner_vec.push(match_catch(bytes)),
                    }
                }
            }
        });
        inner_vec.push(CSVType::Int(idx as i64));
        send.send(inner_vec).unwrap();
    });

    let mut res = receiver.iter()
        .sorted_by_key(|row| 
            match row[row.len() - 1] {
                CSVType::Int(val) => val,
                _ => panic!("Error: Row index didn't populate with an integer"),
            }
        )
        .collect::<Vec<Vec<CSVType>>>();

    for received in 0..res.len() { res[received].pop(); }

    Ok(res)
}

/// Opens a csv file and extract the contents as a vector of [`csv::ByteRecord`].
pub fn csv_read(path: &str) -> Result<Vec<csv::ByteRecord>, CSVPerusalError> {
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
                    Err(e) => return Err(CSVPerusalError::FileError(e)),
                };
            }
            return Ok(data)
        },
        Err(e) => return Err(CSVPerusalError::IOError(e)),
    };
}