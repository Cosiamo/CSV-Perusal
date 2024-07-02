# CSV Perusal

A cargo crate that reads CSV files and returns the contents of the file with the appropriate datatype. Inspired by [Calamine](https://github.com/tafia/calamine/tree/master), this package aims to make dealing with CSV files easier. 

CSV Perusal is built with uploading values from CSV files into relational databases in mind. The value returned when opening a CSV file is a two-dimensional vector of the `CSVType` enumerator, with each inner vector acting as a different row.

CSV Perusal is very simple and easy to use. If you have a file path and just want the values parsed, `open_csv()` is what you're looking for. If you have a particular way you want to open a CSV file with the [`csv`](https://docs.rs/csv/latest/csv/) crate and have a vector of [`ByteRecords`](https://docs.rs/csv/latest/csv/struct.ByteRecord.html), or have a two-dimensional vector of a `String` or `&str`, then you'll want to use the method `get_csv_types()`.

## Example

```rust
use csv_perusal::{open_csv, CSVType};

fn main() {
    let path = "test_data/DATA10ROWS.csv";
    let grid = open_csv(path).unwrap();

    grid.iter().for_each(|row| {
        row.iter().for_each(|cell| {
            match cell {
                CSVType::Int(val) => print!("INT:{:?}, ", val),
                CSVType::Float(val) => print!("FLOAT:{:?}, ", val),
                CSVType::String(val) => print!("STRING:{:?}, ", val),
                CSVType::Date(val) => print!("DATE:{:?}, ", val),
                CSVType::Time(val) => print!("TIME:{:?}, ", val),
                CSVType::DateTime(val) => print!("DATETIME:{:?}, ", val),
                CSVType::Error(err) => print!("ERROR:{:?}", err),
                CSVType::Empty => print!("NONE, "),
            }
        });
        print!("\n");
    });
}
```

### Input

| id | Money | Percent | Date | DateTime | Time12h | Time24h |
| ----- | ----- | ----- | ----- | ----- | ----- | ----- |
| 1	| $8.70 | 34.1% | 5/28/2023 | 10/25/2023 19:48 | 5:31 PM | 23:02 |
| 2 | $6.08 | 90.10% | 2/7/2023 | 11/4/2023 1:58 | 6:47 AM | 14:11 |
| 3 | $6.44  | 50.10% | 7/24/2023 | 7/4/2023 1:04 | 12:32 PM | 17:27 |
| 4 | $4.99  | 15.60% | 12/29/2022 | 12/4/2023 11:34 | 5:17 PM | 4:53 |

### Result

```
STRING: "id", STRING: "Money", STRING: "Percent", STRING: "Date", STRING: "DateTime", STRING: "Time12h", STRING: "Time24h",
-----------------------------
INT: 1, FLOAT: 8.7, FLOAT: 34.1, DATE: "2023-05-28", DATETIME: "2023-10-25 19:48:00", TIME: "17:31:00", TIME: "23:02:00",
INT: 2, FLOAT: 6.08, FLOAT: 90.1, DATE: "2023-02-07", DATETIME: "2023-11-04 01:58:00", TIME: "06:47:00", TIME: "14:11:00",
INT: 3, FLOAT: 6.44, FLOAT: 50.1, DATE: "2023-07-24", DATETIME: "2023-07-04 01:04:00", TIME: "12:32:00", TIME: "17:27:00",
INT: 4, FLOAT: 4.99, FLOAT: 15.6, DATE: "2022-12-29", DATETIME: "2023-12-04 11:34:00", TIME: "17:17:00", TIME: "04:53:00",
```


## Things to Know

- Percentages and currency are converted into floats 
- Times and dates are formatted by [Chrono](https://github.com/chronotope/chrono)
    - It will attempt `mm/dd/yyyy` and `yyyy/mm/dd` formats first but will change to `dd/mm/yyyy` or `yyyy/dd/mm` if month value is greater than 12. Please be aware if you have dates in `dd/mm/yyyy` format, dates such as 2/11/2024 will be read as February 11th instead of November 2nd.
    - Will output dates as `yyyy-mm-dd` but you can use the Chrono package to change the format
    - Currently parses: `mm/dd/yyyy`, `dd/mm/yyyy`, `yyyy/mm/dd`, `yyyy/dd/mm`, `m/dd/yyyy`, `mm/d/yyyy`, `dd/m/yyyy`, `yyyy/mm/d`, `yyyy/dd/m`, `yyyy/m/dd`, `m/d/yyyy`, `yyyy/m/d`, `mm/dd/yy`, `dd/mm/yy`, `m/dd/yy`, `mm/d/yy`, `dd/m/yy`, `yy/m/dd`, `yy/mm/d`, `yy/dd/m`, `m/d/yy`, `yy/m/d` 
- Time is in a 24 hour format but can also be changed to a 12 hour format with Chrono