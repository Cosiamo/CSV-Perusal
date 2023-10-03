extern crate csv_reader;
// use csv::ByteRecord;
use csv_reader::{open::open_csv, csvtype::CSVType};

fn main() {
    let _path = "test_data/Employee_Sample_Data_Limited.csv";
    let path = "test_data/MOCK_DATA.csv";
    let _path = "test_data/DATA10ROWS.csv";
    let _path = "test_data/temp.csv";

    let data = match open_csv(path) {
        Ok(val) => val,
        Err(e) => panic!("{:?}", e),
    };


    for row in data.iter() {
        for column in row.iter() {
            match column {
                CSVType::Int(val) => print!("INT: {:?}, ", val),
                CSVType::Float(val) => print!("FLOAT: {:?}, ", val),
                CSVType::String(val) => print!("STRING: {:?}, ", val),
                CSVType::Date(val) => print!("DATE: {:?}, ", val),
                CSVType::Time(val) => print!("TIME: {:?}, ", val),
                CSVType::DateTime(val) => print!("DATETIME: {:?}, ", val),
                CSVType::Error(e) => panic!("++++++++++++++ERROR++++++++++++++ {:?}", e),
                CSVType::Empty => print!("NONE, "),
            }
        }
        print!("\n");
    }
}
