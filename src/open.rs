use crate::{csvtype::{CSVType, Byte}, utils::match_catch};
use std::{fs::File, path::Path, io::BufReader};
use rayon::prelude::*;
use std::sync::mpsc::channel;
use itertools::Itertools;

pub fn open_csv(path: &str) -> Result<Vec<Vec<CSVType>>, csv::Error> {
    let mut data = match csv_read(path) {
        Ok(data) => data,
        Err(e) => return Err(e),
    };

    let (sender, receiver) = channel();
    data.par_iter_mut().enumerate().for_each_with(sender, |s, (i, item)| {
        let mut inner_vec: Vec<CSVType> = Vec::new();
        item.iter().for_each(|bytes: &[u8]| {
            match bytes {
                [] => inner_vec.push(CSVType::Empty),
                _ => {
                    let byte: Byte<'_> = Byte{b: bytes};
                    match byte {
                        byte if byte.is_number() => inner_vec.push(byte.num_match()),
                        byte if byte.is_dt()=> inner_vec.push(byte.date_and_time()),
                        _ => inner_vec.push(match_catch(bytes)),
                    }
                }
            }
        });
        inner_vec.push(CSVType::Int(i as i64));
        s.send(inner_vec).unwrap();
    });

    let mut res = receiver.iter()
    .sorted_by_key(|x| 
        match x[x.len() - 1] {
            CSVType::Int(v) => v,
            _ => panic!("Error: Row index didn't populate with an integer"),
        }
    )
    .collect::<Vec<Vec<CSVType>>>();

    for r in 0..res.len() { res[r].pop(); }

    Ok(res)
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