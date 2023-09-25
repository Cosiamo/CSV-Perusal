

pub enum CSVType {
    Int(i64),
    Float(f64),
    String(String),
    Date(String),
    Error(csv::Error),
    None(Option<String>),
}

pub fn match_type(data: Vec<csv::ByteRecord>) -> Vec<CSVType> {
    let mut new: Vec<CSVType> = Vec::new();

    for y in 0..data.len() {
        for (_x, bytes) in data[y].iter().enumerate() {
            match bytes {
                [] => new.push(CSVType::None(None)), 
                _ => {
                    let t = String::from_utf8_lossy(&bytes);
                    let s = t.replace(|c: char| !c.is_ascii(), "");
                    match s.parse::<f64>() {
                        Ok(v) => new.push(CSVType::Float(v)),
                        Err(_) => match s.parse::<i64>() {
                            Ok(v) => new.push(CSVType::Int(v)),
                            Err(_) => new.push(match_catch(s))
                        },
                    }
                }
            }
        }
    }

    return new;
}


fn match_catch(s: String) -> CSVType {
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
                Ok(v) => return CSVType::Float(v * 1.0),
                Err(_) => return CSVType::String(s),
            },
            // checks if negative Float
            s if s.chars().map(|x| x).collect::<Vec<char>>()
            .contains(&".".chars().map(|x| x).collect::<Vec<char>>()[0])
            && s.trim().chars().map(|x| x).collect::<Vec<_>>()[0] 
            == "-".chars().map(|x| x).collect::<Vec<_>>()[0] => 
            match s.parse::<f64>() {
                Ok(v) => return CSVType::Float(v * -1.0),
                Err(_) => return CSVType::String(s),
            },
            // checks if negative integer 
            s if s.trim().chars().map(|x| x).collect::<Vec<char>>()[0] 
            == "-".chars().map(|x| x).collect::<Vec<char>>()[0] => 
            match trimmed.parse::<i64>() {
                Ok(v) => return CSVType::Int(v * -1),
                Err(_) => return CSVType::String(s),
            },
            // checks for date
            s if s.chars().map(|x| x).collect::<Vec<char>>()
            .contains(&"/".chars().map(|x| x).collect::<Vec<char>>()[0])
            || s.chars().map(|x| x).collect::<Vec<char>>()
            .contains(&"-".chars().map(|x| x).collect::<Vec<char>>()[0]) => {
                let d = s.trim().to_string();
                match d.len() {
                    10 => {
                        // mm/dd/yyyy
                        if &d[2..3] == "/" && &s[5..6] == "/" 
                        || &d[2..3] == "-" && &d[5..6] == "-" {
                            return CSVType::Date(s);
                        }
                        // yyyy/dd/mm
                        else if &d[4..5] == "/" && &d[7..8] == "/" 
                        || &s[4..5] == "-" && &d[7..8] == "-" {
                            return CSVType::Date(s);
                        } else {
                            return CSVType::String(s);
                        }
                    },
                    9 => {
                        // m/dd/yyyy
                        if &d[1..2] == "/" && &s[4..5] == "/" 
                        || &d[1..2] == "-" && &d[4..5] == "-" {
                            return CSVType::Date(s);
                        }
                        // mm/d/yyyy
                        else if &d[2..3] == "/" && &s[4..5] == "/" 
                        || &d[2..3] == "-" && &d[4..5] == "-" {
                            return CSVType::Date(s);
                        }
                        // yyyy/dd/m
                        else if &d[4..5] == "/" && &d[7..8] == "/" 
                        || &s[4..5] == "-" && &d[7..8] == "-" {
                            return CSVType::Date(s);
                        }
                        // yyyy/d/mm
                        else if &d[4..5] == "/" && &d[6..7] == "/" 
                        || &s[4..5] == "-" && &d[6..7] == "-" {
                            return CSVType::Date(s);
                        } else {
                            return CSVType::String(s);
                        }
                    },
                    // 8 => {},
                    // 7 => {},
                    // 6 => {},
                    _ => return CSVType::String(s),
                }
                
                // m/d/yyyy
            },
            // checks for percent
            s if s.chars().map(|x| x).collect::<Vec<char>>()
            .contains(&"%".chars().map(|x| x).collect::<Vec<char>>()[0])
            => match trimmed.parse::<f64>() {
                Ok(v) => return CSVType::Float((v * 1.0) / 100.0),
                Err(_) => return CSVType::String(s),
            },
            // catches positive integers
            _ => match trimmed.parse::<i64>() {
                Ok(v) => return CSVType::Int(v),
                Err(_) => return CSVType::String(s),
            },
        },
        false => return CSVType::String(s),
    }
}