extern crate csv_reader;
// use csv::ByteRecord;
use csv_reader::{open::open_csv, csvtype::CSVType};

fn main() {
    let _path = "test_data/Employee_Sample_Data_Limited.csv";
    let _path = "test_data/MOCK_DATA.csv";
    let path = "test_data/DATA10ROWS.csv";

    let temp = match open_csv(path) {
        Ok(val) => val,
        Err(e) => panic!("{:?}", e),
    };


    for (_x, v) in temp.iter().enumerate() {
        match v {
            CSVType::Int(i) => print!("INT: {:?}, ", i),
            CSVType::Float(i) => print!("FLOAT: {:?}, ", i),
            CSVType::String(i) => print!("STRING: {:?}, ", i),
            CSVType::Date(i) => print!("DATE: {:?}, ", i),
            CSVType::Time(i) => print!("TIME: {:?}, ", i),
            CSVType::DateTime(i) => print!("DATETIME: {:?}, ", i),
            CSVType::Error(e) => panic!("++++++++++++++ERROR++++++++++++++ {:?}", e),
            CSVType::Empty => print!("NONE, "),
        }
        print!("\n");
    }
}
