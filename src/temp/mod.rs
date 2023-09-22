use std::{fs::File, path::Path, io::BufReader};

pub fn csv_read(path: &str) -> Vec<csv::ByteRecord>{
    let file_handle = match File::open(Path::new(&path)) {
        Ok(val) => val,
        Err(e) => panic!("\u{1b}[31m{:?}\u{1b}[39m", e),
    };
    let reader = BufReader::new(file_handle);
    let mut data = Vec::new();
    // Build the CSV reader and iterate over each record.
    let mut rdr = csv::ReaderBuilder::new()
        .has_headers(false)
        .from_reader(reader);
    for result in rdr.byte_records() {
        match result {
            Ok(record) => data.push(record),
            Err(e) => panic!("\u{1b}[31m{:?}\u{1b}[39m", e),
        };
    }
    
    return data;
}

pub fn csv_transform(data: Vec<csv::ByteRecord>) {
    // println!("{:?}", data);

    for y in 0..data.len() {
        for (x, bytes) in data[y].iter().enumerate() {
            match bytes {
                [] => println!("EMPTY: {:?}", bytes),
                _ => {
                    let t = String::from_utf8_lossy(&bytes);
                    let s = t.replace(|c: char| !c.is_ascii(), "");
                    // checks if unsigned-integer
                    match s.chars().all(char::is_numeric) {
                        true => match s.parse::<u64>() {
                            // matches appropriate bit size
                            Ok(v) => match v {
                                v if v <= std::u8::MAX.into() => 
                                println!("U8: {:?}", v),
                                v if v <= std::u16::MAX.into() => 
                                println!("U16: {:?}", v),
                                v if v <= std::u32::MAX.into() => 
                                println!("U32: {:?}", v),
                                _ => 
                                println!("U64: {:?}", v),
                            },
                            Err(e) => 
                            panic!("\n\u{1b}[31m{:?} \n \u{1b}[30;103mRow:{:?} Col:{:?}\u{1b}[0m\n", 
                                e, y, x
                            ),
                        },
                        false => {
                            let trimmed = s.trim()
                                .replace("$", "")
                                .replace("%", "")
                                .replace(",", "")
                                .replace(".", "")
                                .replace("-", "")
                                .replace("/", "")
                                .replace(":", "");
                            // checks if trimmed value is a number
                            match trimmed.chars().all(char::is_numeric) {
                                true => match s {
                                    // checks if positive Float
                                    s if s.chars().map(|x| x).collect::<Vec<char>>()
                                    .contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0]) =>
                                    match s.parse::<f64>() {
                                        Ok(v) => match v {
                                            v if v <= std::f32::MAX.into() => 
                                            println!("F32: {:?}", v),
                                            _ => 
                                            println!("F64: {:?}", v),
                                        },
                                        Err(e) => 
                                        panic!("\n\u{1b}[31m{:?} \n \u{1b}[30;103mRow:{:?} Col:{:?}\u{1b}[0m\n", 
                                            e, y, x
                                        ),
                                    },
                                    // checks if negative Float
                                    s if s.chars().map(|x| x).collect::<Vec<char>>()
                                    .contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0])
                                    && s.chars().map(|x| x).collect::<Vec<_>>()[0] 
                                    == "-".chars().map(|x| x).collect::<Vec<_>>()[0] => 
                                    match s.parse::<f64>() {
                                        Ok(v) => match v {
                                            v if (v * -1.0) >= std::f32::MIN.into() => 
                                            println!("F32: {:?}", v),
                                            _ => 
                                            println!("F64: {:?}", v),
                                        }
                                        Err(e) => 
                                        panic!("\n\u{1b}[31m{:?} \n \u{1b}[30;103mRow:{:?} Col:{:?}\u{1b}[0m\n", 
                                            e, y, x
                                        ),
                                    },
                                    // checks if negative integer 
                                    s if s.chars().map(|x| x).collect::<Vec<char>>()[0] 
                                    == "-".chars().map(|x| x).collect::<Vec<char>>()[0] => 
                                    match trimmed.parse::<i64>() {
                                        Ok(v) => match v {
                                            v if (v * -1) >= std::i8::MIN.into() => 
                                            println!("i8: {:?}", v),
                                            v if (v * -1) >= std::i16::MIN.into() => 
                                            println!("i16: {:?}", v),
                                            v if (v * -1) >= std::i32::MIN.into() => 
                                            println!("i32: {:?}", v),
                                            _ => 
                                            println!("i64: {:?}", v),
                                        },
                                        Err(e) => 
                                        panic!("\n\u{1b}[31m{:?} \n \u{1b}[30;103mRow:{:?} Col:{:?}\u{1b}[0m\n", 
                                            e, y, x
                                        ),
                                    },
                                    // catches unsigned integers
                                    _ => match trimmed.parse::<u64>() {
                                        Ok(v) => {
                                            // println!("U64:{:?}, BYTES:{:?} \u{1b}[30;103mRow:{:?} Col:{:?}\u{1b}[0m\n", trimmed, bytes, (num + y) + 1, x + 1);
                                            match v {
                                                v if v <= std::u8::MAX.into() => 
                                                println!("U8: {:?}", v),
                                                v if v <= std::u16::MAX.into() => 
                                                println!("U16: {:?}", v),
                                                v if v <= std::u32::MAX.into() => 
                                                println!("U32: {:?}", v),
                                                _ => 
                                                println!("U64: {:?}", v),
                                        }},
                                        Err(e) => 
                                                panic!("\n\u{1b}[31m{:?} \n \u{1b}[30;103mRow:{:?} Col:{:?}\u{1b}[0m\n", 
                                                    e, y, x
                                                ),
                                    },
                                },
                                false => println!("CATCH: {:?}", trimmed),
                            }
                        },
                    }
                },
            }
        };
    }
}

// fn is_byte_string(byte: &[u8]) -> bool {
//     let t = String::from_utf8_lossy(byte);
//     let s = t.replace(|c: char| !c.is_ascii(), "");
//     match s.chars().all(char::is_numeric) {
//         true => return false,
//         false => {
//             let trimmed = s.trim()
//                 .replace("$", "")
//                 .replace("%", "")
//                 .replace(",", "")
//                 .replace(".", "")
//                 .replace("-", "")
//                 .replace("/", "")
//                 .replace(":", "");
//             // checks if trimmed value is a number
//             match trimmed.chars().all(char::is_numeric) {
//                 true => return false,
//                 false => return true,
//             }
//         },
//     }
// }