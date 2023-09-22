extern crate csv_reader;
use csv_reader::temp::{csv_read, csv_transform};

fn main() {
    let val = csv_read("test_data/data100.csv");

    csv_transform(val);
}
