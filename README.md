# CSV Perusal

A cargo crate that reads CSV files and returns the contents of the file with the appropriate datatype. Inspired by [Calamine](https://github.com/tafia/calamine/tree/master), this package aims to make dealing with CSV files easier. 

CSV Perusal is built with uploading values from CSV files into relational databases in mind. The value returned when opening a CSV file is a 2D vector, with each inner vector acting as a different row.

## Things to Know

- Percentages and currency are converted into floats 
- Times and dates are formatted by [Chrono](https://github.com/chronotope/chrono)
    - It will attempt `mm/dd/yyyy` and `yyyy/mm/dd` formats first but will change to `dd/mm/yyyy` or `yyyy/dd/mm` if month value is greater than 12. Please be aware if you have dates in `dd/mm/yyyy` format, dates such as 2/11/2024 will be read as February 11th instead of November 2nd.
    - Will output dates as `yyyy-mm-dd` but you can use the chrono package to change the format
    - Currently parses: `mm/dd/yyyy`, `dd/mm/yyyy`, `yyyy/mm/dd`, `yyyy/dd/mm`, `m/dd/yyyy`, `mm/d/yyyy`, `dd/m/yyyy`, `yyyy/mm/d`, `yyyy/dd/m`, `yyyy/m/dd`, `m/d/yyyy`, `yyyy/m/d`, `mm/dd/yy`, `dd/mm/yy`, `m/dd/yy`, `mm/d/yy`, `dd/m/yy`, `yy/m/dd`, `yy/mm/d`, `yy/dd/m`, `m/d/yy`, `yy/m/d` 
- Time is in a 24 hour format

## Example

```rust
use csv_perusal::{open::open_csv, csvtype::CSVType};

fn main() {
    let path = "test_data/MOCK_DATA.csv";

    let data = open_csv(path).unwrap();

    for (y, row) in data.iter().enumerate() {
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
        if y == 0 {
            println!("-----------------------------");
        }
    }
}

```

### Result

```
STRING: "Employee ID", STRING: "Job Title", STRING: "Department", STRING: "Business Unit", STRING: "Hire Date", STRING: "Annual Salary", STRING: "Bonus %", STRING: "Country", STRING: "City", STRING: "Exit Date",
-----------------------------
STRING: "E02002", STRING: "Controls Engineer", STRING: "Engineering", STRING: "Manufacturing", DATE: "2022-02-05", FLOAT: 92368.0, FLOAT: 0.0, STRING: "United States", STRING: "Columbus", NONE,
STRING: "E02003", STRING: "Analyst", STRING: "Sales", STRING: "Corporate", DATE: "2013-10-23", FLOAT: 45703.0, FLOAT: 0.0, STRING: "United States", STRING: "Chicago", NONE,
STRING: "E02004", STRING: "Network Administrator", STRING: "IT", STRING: "Research & Development", DATE: "2019-03-24", FLOAT: 83576.0, FLOAT: 0.0, STRING: "China", STRING: "Shanghai", NONE,
STRING: "E02005", STRING: "IT Systems Architect", STRING: "IT", STRING: "Corporate", DATE: "2018-04-07", FLOAT: 98062.0, FLOAT: 0.0, STRING: "United States", STRING: "Seattle", NONE,
STRING: "E02006", STRING: "Director", STRING: "Engineering", STRING: "Corporate", DATE: "2005-06-18", FLOAT: 175391.0, FLOAT: 24.0, STRING: "United States", STRING: "Austin", NONE,
STRING: "E02007", STRING: "Network Administrator", STRING: "IT", STRING: "Manufacturing", DATE: "2004-04-22", FLOAT: 66227.0, FLOAT: 0.0, STRING: "United States", STRING: "Phoenix", DATE: "2014-02-14",
STRING: "E02008", STRING: "Sr. Analyst", STRING: "Accounting", STRING: "Specialty Products", DATE: "2009-06-27", FLOAT: 89744.0, FLOAT: 0.0, STRING: "China", STRING: "Chongqing", NONE,
STRING: "E02009", STRING: "Analyst II", STRING: "Finance", STRING: "Corporate", DATE: "1999-02-19", FLOAT: 69674.0, FLOAT: 0.0, STRING: "China", STRING: "Chengdu", NONE,
STRING: "E02010", STRING: "System Administrator", STRING: "IT", STRING: "Manufacturing", DATE: "2011-09-09", FLOAT: 97630.0, FLOAT: 0.0, STRING: "United States", STRING: "Seattle", NONE,
STRING: "E02011", STRING: "Manager", STRING: "Finance", STRING: "Specialty Products", DATE: "2015-02-05", FLOAT: 105879.0, FLOAT: 10.0, STRING: "United States", STRING: "Miami", NONE,
```