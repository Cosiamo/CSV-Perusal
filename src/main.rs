extern crate csv_reader;
use csv_reader::open::csv_read;
use csv_reader::csvtype::{CSVType, match_type};
use csv::ByteRecord;

fn main() {
    let path = "test_data/Employee_Sample_Data_Limited.csv";
    let _path = "test_data/MOCK_DATA.csv";

    let val: Vec<ByteRecord> = csv_read(path);

    let temp = match_type(val);
    for v in temp.iter() {
        match v {
            CSVType::Int(i) => println!("INT: {:?}", i),
            CSVType::Float(i) => println!("FLOAT: {:?}", i),
            CSVType::String(i) => println!("STRING: {:?}", i),
            CSVType::Date(i) => println!("DATE: {:?}", i),
            CSVType::Error(_) => println!("++++++++++++++ERROR++++++++++++++"),
            CSVType::None(i) => println!("NONE: {:?}", i),
        }
    }
}
