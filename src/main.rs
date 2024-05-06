#![allow(unused)]

extern crate csv_perusal;
use csv::ByteRecord;
use csv_perusal::
{open::open_csv,csvtype::CSVType};

fn main() {
    let _path = "test_data/Employee_Sample_Data_Limited.csv";
    let _path = "test_data/MOCK_DATA.csv";
    let _path = "test_data/DATA10ROWS.csv";
    let _path = "test_data/temp.csv";
    let _path = "test_data/large_dataset.csv";
    let path = "test_data/mil_rows.csv";
    let _path = "C:/Users/UMS/Downloads/UMS_Daily_Feedback_Report_10102023.csv";

    let _data = open_csv(path).unwrap();

    // let mut counter = 1;
    // data.iter().enumerate().for_each(|(i, cell)| {
    //     match cell {
    //         CSVType::Int(val) => print!("INT: {:?}, ", val),
    //         CSVType::Float(val) => print!("FLOAT: {:?}, ", val),
    //         CSVType::String(val) => print!("STRING: {:?}, ", val),
    //         CSVType::Date(val) => print!("DATE: {:?}, ", val),
    //         CSVType::Time(val) => print!("TIME: {:?}, ", val),
    //         CSVType::DateTime(val) => print!("DATETIME: {:?}, ", val),
    //         CSVType::Error(e) => panic!("++++++++++++++ERROR++++++++++++++ {:?}", e),
    //         CSVType::Empty => print!("NONE, "),
    //     }
    //     if i == 0 {
    //         print!("");
    //     } else if width * counter == (i + 1) {
    //         if width == (i + 1) {
    //             print!("\n-----------------------------");
    //         }
    //         println!("\n");
    //         counter += 1;
    //     }
    // })

    // for (y, row) in data.iter().enumerate() {
    //     for column in row.iter() {
    //         match column {
    //             CSVType::Int(val) => print!("INT: {:?}, ", val),
    //             CSVType::Float(val) => print!("FLOAT: {:?}, ", val),
    //             CSVType::String(val) => print!("STRING: {:?}, ", val),
    //             CSVType::Date(val) => print!("DATE: {:?}, ", val),
    //             CSVType::Time(val) => print!("TIME: {:?}, ", val),
    //             CSVType::DateTime(val) => print!("DATETIME: {:?}, ", val),
    //             CSVType::Error(e) => panic!("++++++++++++++ERROR++++++++++++++ {:?}", e),
    //             CSVType::Empty => print!("NONE, "),
    //         }
    //     }
    //     print!("\n");
    //     if y == 0 {
    //         println!("-----------------------------");
    //     }
    // }
}
