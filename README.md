# CSV Perusal

A cargo crate that reads CSV files and returns the contents of the file with the appropriate datatype. Inspired by [Calamine](https://github.com/tafia/calamine/tree/master), this package aims to make dealing with CSV files easier. 

CSV Perusal is built with uploading values from CSV files into relational databases in mind. The value returned when opening a CSV file is a 2D vector, with each inner vector acting as a different row.

CSV Perusal is very simple and easy to use. You only need to know one function and one enumerator. 
The `open_csv()` function returns a 2d vector of the `CSVType` enumerator which returns any of the following:
- Int(i64)
- Float(f64)
- String(String)
- Date(String)
- Time(String)
- DateTime(String)
- Error(String) <b><u>THIS IS TEMPORARY</u></b>
- Empty

## Example

```rust
use csv_perusal::{open::open_csv, csvtype::CSVType};

fn main() {
    let path = "test_data/MOCK_DATA.csv";

    let data = open_csv(path).unwrap();

    for (y, row) in data.iter().enumerate() {
        for cell in row.iter() {
            match cell {
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
- The Error in the CSVType enumerator is `std::convert::Infallible` which is used when there's an issue parsing datatypes. The only other error is if the <i>path</i> for open_csv() is invalid.




------------------------------------------------------------------------------------

# Faster Ways To Iterate and Assign Types to 2D Vectors

I was recently thinking about a project I worked on a few months ago to reflect on the things I've learned about Rust since then. 
One thing that I do a lot of for work is create and iterate over 2D vectors to filter data and assign the appropriate data types to values for our SQL data base. 
So the project is a cargo crate that uses the CSV crate to open a CSV file then assigns data types to the values. 
I was still relatively new to the language and wanted to get this task done quickly, so I did the simple thing and made nested for loops to get the values. 
Nothing wrong with this approach, but as you'll see in the benchmarks, it isn't as fast as it could be.

# Nested Regular For Loops
```rust
let mut outer_vec: Vec<Vec<CSVType>> = Vec::new();
for y in 0..data.len() {
    let mut inner_vec: Vec<CSVType> = Vec::new();
    for bytes: &[u8] in data[y].iter() {
        match bytes {
            [] => inner_vec.push(CSVType::Empty),
            _ => {
                let byte: Byte<'_> = Byte{b: bytes};
                match byte {
                    byte if byte.is_number() => inner_vec.push(byte.num_match()),
                    byte if byte.is_dt() => inner_vec.push(byte.date_and_time()),
                    _ => inner_vec.push(match_catch(bytes)),
                }
            }
        }
    }
    outer_vec.push(inner_vec);
}

Ok(outer_vec)
```

The first (and probably obvious) thing to do is to use the rayon crate to replace `.iter()` with `.par_mut_iter()` to iterate the data in parallel. 
I also replaced the inner loop with `.for_each()` because I read that it some cases that it could be faster, however the performance didn't change.
But, I do prefer this syntax, so I kept it.

# Use Parallel Iteration and Replaced Outside Vector with Channels
```rust
let (sender, receiver) = channel();
data.par_iter_mut().for_each_with(sender, |s, item| {
    let mut inner_vec: Vec<CSVType> = Vec::new();
    // item is a &ByteRecord and can't be iterated in parallel
    item.iter().for_each(|bytes: &[u8]| {
        match bytes {
            [] => inner_vec.push(CSVType::Empty),
            _ => {
                let byte: Byte<'_> = Byte{b: bytes};
                match byte {
                    byte if byte.is_number() => inner_vec.push(byte.num_match()),
                    byte if byte.is_dt() => inner_vec.push(byte.date_and_time()),
                    _ => inner_vec.push(match_catch(bytes)),
                }
            }
        }
    });
    s.send(inner_vec).unwrap();
});

let res = receiver.iter().collect::<Vec<Vec<CSVType>>>();
Ok(res)
```

This approach did increase performance significantly, but the rows were all out of order, which could be a major problem if the header isn't received at index 0. 
So I decided to enumerate the outer iteration, push it at the end of the inner vectors, sort them on that column once they're received, then pop it before the data gets returned.

# Parallel Iteration with an Index
```rust 
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
                    byte if byte.is_dt() => inner_vec.push(byte.date_and_time()),
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

for i in 0..res.len() { res[i].pop(); }

Ok(res)
```

# Benchmarks

For the benchmarks I used hyperfine and ran everything as `cargo run --release` with 1 warmup. 
As you'll see, the parallel iterator is slower on very small datasets, but has massive performance gains as the size of the data increases. 
I believe that's because it takes longer to create the channels and pass the data through the threads than initialize a few vectors, but I'm not entirely sure.

## 10 Rows

#### Nested For Loop
```
  Time (mean ± σ):     106.1 ms ±   8.6 ms    [User: 11.7 ms, System: 8.4 ms]
  Range (min … max):    95.2 ms … 141.0 ms    100 runs
```

#### Parallel Iteration
```
  Time (mean ± σ):     117.1 ms ±  11.0 ms    [User: 11.1 ms, System: 8.8 ms]
  Range (min … max):    99.4 ms … 144.7 ms    100 runs
```

#### Parallel Iteration with Index
```
  Time (mean ± σ):     112.9 ms ±  10.1 ms    [User: 7.5 ms, System: 8.9 ms]
  Range (min … max):    98.2 ms … 140.5 ms    100 runs
```

-------------------------------

## 16,000 Rows

#### Nested For Loop
```
  Time (mean ± σ):     281.3 ms ±  12.5 ms    [User: 151.2 ms, System: 15.5 ms]
  Range (min … max):   258.7 ms … 338.2 ms    100 runs
```

#### Parallel Iteration
```
  Time (mean ± σ):     171.4 ms ±  10.2 ms    [User: 335.2 ms, System: 11.2 ms]
  Range (min … max):   153.3 ms … 198.4 ms    100 runs
```

#### Parallel Iteration with Index
```
  Time (mean ± σ):     176.1 ms ±  12.2 ms    [User: 345.8 ms, System: 9.9 ms]
  Range (min … max):   155.5 ms … 230.7 ms    100 runs
```

-------------------------------

## 1.24 Million Rows

#### Nested For Loop
```
  Time (mean ± σ):     12.910 s ±  1.561 s    [User: 7.767 s, System: 0.312 s]
  Range (min … max):   11.090 s … 15.813 s    10 runs
```

#### Parallel Iteration
```
  Time (mean ± σ):      3.894 s ±  0.166 s    [User: 30.308 s, System: 0.670 s]
  Range (min … max):    3.663 s …  4.191 s    10 runs
```

#### Parallel Iteration with Index
```
  Time (mean ± σ):      4.352 s ±  0.112 s    [User: 31.134 s, System: 0.664 s]
  Range (min … max):    4.190 s …  4.539 s    10 runs
```