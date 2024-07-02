use std::{fs::File, path::Path, io::BufReader};

use crate::errors::CSVPerusalError;

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