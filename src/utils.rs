use crate::{errors::CSVPerusalError, numbers::utils::match_catch, types::Byte, CSVType};
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

/// Converts a two-dimensional vector into a vector of [`csv::ByteRecord`].
/// 
/// The macro joins the inner vector via a tab, then converts the input data into a [`String`].
/// This creates a tab-separated-value grid that is be used to build a vector of [`csv::ByteRecord`].
#[macro_export]
macro_rules! grid_to_byterecord {
    ($data:expr_2021) => {{
        let tabbed = $data.iter().map(|row| row.join("\t")).collect::<Vec<String>>();
        let tsv = tabbed.join("\n");
        let mut reader = ReaderBuilder::new().delimiter(b'\t').from_reader(tsv.as_bytes());
        let mut byterecord = Vec::new(); 
        for result in reader.byte_records() { byterecord.push(result?) }
        byterecord
    }};
}